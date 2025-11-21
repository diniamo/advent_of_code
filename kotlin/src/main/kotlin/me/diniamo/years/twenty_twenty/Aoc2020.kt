package me.diniamo.years.twenty_twenty

import me.diniamo.Day
import me.diniamo.Year

class Aoc2020 : Year(2020) {
    override val days: Collection<Day<out Any>>
        get() = listOf(
            Day1(year),
            Day2(year),
            Day3(year),
            Day4(year)
        )
}
