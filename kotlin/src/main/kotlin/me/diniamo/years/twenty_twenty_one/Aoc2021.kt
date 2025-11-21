package me.diniamo.years.twenty_twenty_one

import me.diniamo.Day
import me.diniamo.Year

class Aoc2021 : Year(2021) {
    override val days: Collection<Day<out Any>>
        get() = listOf(
            //Day1(year),
            //Day2(year),
            //Day3(year),
            //Day4(year),
            //Day5(year),
            //Day6(year),
            Day7(year)
        )
}