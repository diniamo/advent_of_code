package me.diniamo.years.twenty_twenty

import me.diniamo.Day

class Day3(year: Int) : Day<List<String>>(3, year) {
    override fun partOne(input: List<String>): Any = calc(input, 3, 1)
    override fun partTwo(input: List<String>): Any =
        calc(input, 1, 1) * calc(input, 3, 1) * calc(input, 5, 1) * calc(input, 7, 1) * calc(input, 1, 2)

    private fun calc(input: List<String>, xStep: Int, yStep: Int): Long {
        var toReturn: Long = 0

        var i = yStep
        var x = 0
        while (i < input.size) {
            x += xStep
            x %= input[i].length

            if (input[i][x] == '#') {
                toReturn++
            }

            i += yStep
        }

        return toReturn
    }

    override fun adaptInput(input: List<String>) = input
}