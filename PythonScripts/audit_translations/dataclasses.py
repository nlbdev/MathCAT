"""
Data models for the audit tool.

Contains dataclasses for representing rules and comparison results.
"""

from dataclasses import dataclass, field
from typing import Any


@dataclass
class RuleInfo:
    """
    Information about a single rule parsed from a YAML file.

    Attributes
    ----------
    name : str | None
        Rule name for standard rule files; None for unicode entries.
    tag : str | None
        Rule tag (normalized string); None for unicode entries.
    key : str
        Stable identifier used for matching; for unicode entries this is the character or range key.
    line_number : int
        1-based line number where the rule starts in the source file.
    raw_content : str
        Raw YAML block for this rule (used for reporting/snippets).
    data : Any | None
        Parsed YAML node for the rule; used for structural diffs.
    untranslated_entries : list[tuple[str, str, int | None]]
        List of (key, text, line) entries extracted from lowercase translation keys.
        This drives per-issue JSONL output so each untranslated string can report
        the specific YAML line number where it appears.
    line_map : dict[str, list[int]]
        Mapping of element type to line numbers for rule components like match,
        conditions, variables, and structural tokens. This is used to point
        structural diffs at a precise line rather than the top of the rule.
    audit_ignore : bool
        True if the raw content contains an audit-ignore marker.
    """

    name: str | None  # None for unicode entries
    tag: str | None  # None for unicode entries
    key: str  # For unicode entries, this is the character/range
    line_number: int
    raw_content: str
    data: Any | None = None
    untranslated_entries: list[tuple[str, str, int | None]] = field(default_factory=list)
    line_map: dict[str, list[int]] = field(default_factory=dict)
    audit_ignore: bool = False

    @property
    def has_untranslated_text(self) -> bool:
        return bool(self.untranslated_entries)

    @property
    def untranslated_keys(self) -> list[str]:
        return [entry[1] for entry in self.untranslated_entries]


@dataclass
class RuleDifference:
    """Fine-grained difference between English and translated rule"""

    english_rule: RuleInfo
    translated_rule: RuleInfo
    diff_type: str  # 'match', 'condition', 'structure', 'variables'
    description: str
    english_snippet: str
    translated_snippet: str


@dataclass
class ComparisonResult:
    """Results from comparing English and translated files"""

    missing_rules: list[RuleInfo]  # Rules in English but not in translation
    extra_rules: list[RuleInfo]  # Rules in translation but not in English
    untranslated_text: list[tuple[RuleInfo, list[tuple[str, str, int | None]]]]  # Rules with lowercase t/ot/ct
    file_path: str
    english_rule_count: int
    translated_rule_count: int
    rule_differences: list[RuleDifference] = field(default_factory=list)  # Fine-grained diffs

    @property
    def has_issues(self) -> bool:
        return bool(self.missing_rules or self.untranslated_text or self.extra_rules or self.rule_differences)
