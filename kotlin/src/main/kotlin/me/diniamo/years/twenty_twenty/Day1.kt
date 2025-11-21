package me.diniamo.years.twenty_twenty

import me.diniamo.Day

class Day1(year: Int) : Day<List<Int>>(1, year) {
    override fun partOne(input: List<Int>): Any {
        var result: Int = -1
        for(i in input) {
            for(j in input) {
                if(i + j == 2020) {
                    result = i * j
                }
            }
        }

        return result
    }

    override fun partTwo(input: List<Int>): Any {
        var result: Int = -1
        for(i in input) {
            for(j in input) {
                for(k in input)
                    if(i + j + k == 2020) {
                        result = i * j * k
                    }
            }
        }

        return result
    }

    override fun adaptInput(input: List<String>): List<Int> = input.map(String::toInt)
}