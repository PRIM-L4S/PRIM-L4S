import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import re

_SUPERSCRIPT_DIGITS = str.maketrans("0123456789-+", "⁰¹²³⁴⁵⁶⁷⁸⁹⁻⁺")
_OFFSET_FONT_SIZE = 12


class _SuperscriptScalarFormatter(mticker.ScalarFormatter):
    def get_offset(self) -> str:
        offset = super().get_offset()
        match = re.fullmatch(r"([+-]?)(\d+(?:\.\d+)?)e([+-]?\d+)", offset)
        if match and match.group(2) == "1":
            sign = match.group(1)
            exponent = match.group(3).translate(_SUPERSCRIPT_DIGITS)
            return f"{sign}10{exponent}"
        return offset


def format_y_axis_as_scientific_notation() -> None:
    formatter = _SuperscriptScalarFormatter()
    formatter.set_scientific(True)
    formatter.set_powerlimits((0, 0))
    axis = plt.gca().yaxis
    axis.set_major_formatter(formatter)
    axis.get_offset_text().set_fontsize(_OFFSET_FONT_SIZE)
