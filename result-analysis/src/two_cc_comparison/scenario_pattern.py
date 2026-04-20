import re


def match_pattern(
    pattern: re.Pattern, scenario: str, cc1: str
) -> tuple[int, int] | None:
    match = pattern.match(scenario)
    if not match:
        return None

    single_cc = match.group("single_cc")
    if single_cc:
        single_n = int(match.group("single_n"))
        if single_cc == cc1:
            return single_n, 0
        return 0, single_n

    left_cc = match.group("left_cc")
    right_cc = match.group("right_cc")

    # Reject cases like "3prague+7prague@..."
    if left_cc == right_cc:
        raise ValueError(
            f"Invalid scenario '{scenario}': both sides have the same congestion control algorithm '{left_cc}'."
        )

    left_n = int(match.group("left_n"))
    right_n = int(match.group("right_n"))

    # Always map counts to cc1/cc2 correctly, regardless of side.
    if left_cc == cc1:
        nbr_cc1, nbr_cc2 = left_n, right_n
    else:
        nbr_cc1, nbr_cc2 = right_n, left_n

    return nbr_cc1, nbr_cc2


def get_two_cc_scenario_pattern(
    cc1: str,
    cc2: str,
    other_params: str,
) -> re.Pattern:
    return re.compile(
        rf"^(?:"
        rf"(?P<single_n>\d+)(?P<single_cc>{re.escape(cc1)}|{re.escape(cc2)})"
        rf"|"
        rf"(?P<left_n>\d+)(?P<left_cc>{re.escape(cc1)}|{re.escape(cc2)})\+"
        rf"(?P<right_n>\d+)(?P<right_cc>{re.escape(cc1)}|{re.escape(cc2)})"
        rf")@{re.escape(other_params)}$"
    )
