package me.diniamo.years.twenty_twenty_one

import me.diniamo.Day

class Day1(year: Int) : Day<List<Int>>(1, year) {
    override fun partOne(input: List<Int>): Any {
        var previous = 0
        var incCount = 0

        for(i in input.indices) {
            val current = input[i]

            if(i == 0) {
                // No previous measure to look at
                previous = current
                continue
            }

            if(input[i] > previous)
                incCount++

            previous = current
        }

        return incCount
    }

    override fun partTwo(input: List<Int>): Any {
        var previousSum = 0
        var incCount = 0

        for(i in input.indices) {
            val currentSum = input[i] + input[i + 1] + input[i + 2]

            if(i == 0) {
                previousSum = currentSum
                continue
            }

            if(currentSum > previousSum)
                incCount++

            if(i == input.size - 3)
                break

            previousSum = currentSum
        }

        return incCount
    }

    override fun adaptInput(input: List<String>): List<Int> = input.map { it.toInt() }
}