"""
Rich console rendering and issue serialization.

Handles all output concerns: rich terminal display, JSONL issue normalization,
and the IssueWriter interface.
"""

from pathlib import Path
from typing import Any

from rich.console import Console
from rich.markup import escape

from .dataclasses import ComparisonResult, DiffType, IssueType, RuleDifference, RuleInfo
from .line_resolver import resolve_diff_lines

console = Console()


type IssueGroupKey = tuple[IssueType, DiffType | None]


def issue_group_key(issue_type: IssueType, diff_type: DiffType | None = None) -> IssueGroupKey:
    return issue_type, diff_type


ISSUE_GROUP_SPECS: list[tuple[IssueGroupKey, str]] = [
    (issue_group_key(IssueType.MISSING_RULE), "Missing in Translation"),
    (issue_group_key(IssueType.UNTRANSLATED_TEXT), "Untranslated Text"),
    (issue_group_key(IssueType.RULE_DIFFERENCE, DiffType.MATCH), "Match Pattern Differences"),
    (issue_group_key(IssueType.RULE_DIFFERENCE, DiffType.CONDITION), "Condition Differences"),
    (issue_group_key(IssueType.RULE_DIFFERENCE, DiffType.VARIABLES), "Variable Differences"),
    (issue_group_key(IssueType.RULE_DIFFERENCE, DiffType.STRUCTURE), "Structure Differences"),
    (issue_group_key(IssueType.EXTRA_RULE), "Extra in Translation"),
]
ISSUE_GROUP_LABELS = {key: label for key, label in ISSUE_GROUP_SPECS}


def rule_label(rule: RuleInfo) -> str:
    if rule.name is None:
        return f'[yellow]"{escape(rule.key)}"[/]'
    tag = rule.tag or "unknown"
    return f"[cyan]{escape(rule.name)}[/] [dim]({escape(tag)})[/]"


def issue_base(rule: RuleInfo, file_name: str, language: str) -> dict:
    return {
        "language": language,
        "file": Path(file_name).as_posix(),
        "rule_name": rule.name or "",
        "rule_tag": rule.tag or "",
        "rule_key": rule.key,
        "issue_line_en": None,
        "issue_line_tr": None,
        "rule_line_en": None,
        "rule_line_tr": None,
    }


def collect_issues(
    result: ComparisonResult,
    file_name: str,
    language: str,
) -> list[dict]:
    """
    Flatten a ComparisonResult into one normalized dictionary per issue.

    This is the canonical bridge from parser/diff objects to serializable
    records consumed by JSONL output, snapshot tests, and line-level assertions.
    """
    issues = []

    for rule in result.missing_rules:
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type=IssueType.MISSING_RULE.value,
            diff_type="",
            issue_line_en=rule.line_number,
            rule_line_en=rule.line_number,
            description="Rule present in English but missing in translation",
            english_snippet="",
            translated_snippet="",
            untranslated_texts=[],
        )
        issues.append(issue)

    for rule in result.extra_rules:
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type=IssueType.EXTRA_RULE.value,
            diff_type="",
            issue_line_tr=rule.line_number,
            rule_line_tr=rule.line_number,
            description="Rule present in translation but missing in English",
            english_snippet="",
            translated_snippet="",
            untranslated_texts=[],
        )
        issues.append(issue)

    for rule, entries in result.untranslated_text:
        for _key, text, line in entries:
            issue = issue_base(rule, file_name, language)
            issue.update(
                issue_type=IssueType.UNTRANSLATED_TEXT.value,
                diff_type="",
                issue_line_tr=line or rule.line_number,
                rule_line_tr=rule.line_number,
                description="Lowercase t/ot/ct keys indicate untranslated text",
                english_snippet="",
                translated_snippet="",
                untranslated_texts=[text],
            )
            issues.append(issue)

    for diff in result.rule_differences:
        lines = resolve_diff_lines(diff)
        if lines is None:
            continue
        issue_line_en, issue_line_tr = lines
        issue = issue_base(diff.english_rule, file_name, language)
        issue.update(
            issue_type=IssueType.RULE_DIFFERENCE.value,
            diff_type=diff.diff_type.value,
            issue_line_en=issue_line_en,
            issue_line_tr=issue_line_tr,
            rule_line_en=diff.english_rule.line_number,
            rule_line_tr=diff.translated_rule.line_number,
            description=diff.description,
            english_snippet=diff.english_snippet,
            translated_snippet=diff.translated_snippet,
            untranslated_texts=[],
        )
        issues.append(issue)

    return issues


def print_warnings(
    result: ComparisonResult,
    file_name: str,
    verbose: bool = False,
    target_language: str = "tr",
) -> int:
    """Print warnings to console. Returns count of issues found."""
    issues = 0
    display_name = Path(file_name).as_posix()
    target_label = target_language.lower().replace("_", "-")

    if not result.has_issues:
        return issues

    style, icon = (
        ("green", "✓")
        if result.translated_rule_count == result.english_rule_count
        else ("red", "✗")
        if result.translated_rule_count == 0
        else ("yellow", "⚠")
    )
    console.print()
    console.rule(style="cyan")
    console.print(f"[{style}]{icon}[/] [bold]{escape(display_name)}[/]")
    console.print(f"  [dim]English: {result.english_rule_count} rules  →  Translated: {result.translated_rule_count} rules[/]")
    console.rule(style="cyan")

    grouped_issues: dict[str, dict[str, Any]] = {}

    def add_issue(rule: RuleInfo, group_key: IssueGroupKey, payload: dict[str, Any]) -> None:
        if rule.key not in grouped_issues:
            grouped_issues[rule.key] = {"rule": rule, "by_type": {}}
        grouped_issues[rule.key]["by_type"].setdefault(group_key, []).append(payload)

    for rule in result.missing_rules:
        add_issue(rule, issue_group_key(IssueType.MISSING_RULE), {"line_en": rule.line_number})

    for rule, entries in result.untranslated_text:
        for _, text, line in entries:
            add_issue(rule, issue_group_key(IssueType.UNTRANSLATED_TEXT), {"line_tr": line or rule.line_number, "text": text})

    for diff in result.rule_differences:
        lines = resolve_diff_lines(diff)
        if lines is None:
            continue
        line_en, line_tr = lines
        add_issue(
            diff.english_rule,
            issue_group_key(IssueType.RULE_DIFFERENCE, diff.diff_type),
            {"line_en": line_en, "line_tr": line_tr, "diff": diff},
        )

    for rule in result.extra_rules:
        add_issue(rule, issue_group_key(IssueType.EXTRA_RULE), {"line_tr": rule.line_number})

    if grouped_issues:
        total_grouped_issues = sum(len(entries) for group in grouped_issues.values() for entries in group["by_type"].values())
        console.print(
            f"\n  [magenta]≠[/] [bold]Rule Issues[/] "
            f"[[magenta]{total_grouped_issues}[/]] [dim](grouped by rule and issue type)[/]"
        )
        for group in grouped_issues.values():
            rule = group["rule"]
            by_type: dict[IssueGroupKey, list[dict[str, Any]]] = group["by_type"]
            console.print(f"      [dim]•[/] {rule_label(rule)}")
            ordered_group_keys = [group_key for group_key, _ in ISSUE_GROUP_SPECS if group_key in by_type]
            for group_key in ordered_group_keys:
                entries = by_type[group_key]
                issue_type, _diff_type = group_key
                console.print(f"          [dim]{ISSUE_GROUP_LABELS[group_key]} [{len(entries)}][/]")
                for entry in entries:
                    if issue_type is IssueType.MISSING_RULE:
                        console.print(f"              [dim]•[/] [dim](line {entry['line_en']} in English)[/]")
                    elif issue_type is IssueType.EXTRA_RULE:
                        console.print(f"              [dim]•[/] [dim](line {entry['line_tr']} in {target_label})[/]")
                    elif issue_type is IssueType.UNTRANSLATED_TEXT:
                        console.print(
                            f"              [dim]•[/] [dim](line {entry['line_tr']} {target_label})[/] "
                            f'[yellow]"{escape(entry["text"])}"[/]'
                        )
                    else:
                        diff: RuleDifference = entry["diff"]
                        console.print(
                            f"              [dim]•[/] [dim](line {entry['line_en']} en, {entry['line_tr']} {target_label})[/]"
                        )
                        console.print(f"                  [dim]{diff.description}[/]")
                        if verbose:
                            console.print(f"                  [green]en:[/] {escape(diff.english_snippet)}")
                            console.print(f"                  [red]{target_label}:[/] {escape(diff.translated_snippet)}")
                issues += len(entries)

    return issues
