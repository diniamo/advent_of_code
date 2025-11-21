package me.diniamo.years.twenty_twenty_one

import me.diniamo.Day

class Day2(year: Int) : Day<List<Pair<String, Int>>>(2, year) {
    override fun partOne(input: List<Pair<String, Int>>): Any {
        var x = 0
        var y = 0

        for(p in input) {
            when(p.first) {
                "forward" -> x += p.second
                "down" -> y += p.second
                "up" -> y -= p.second
            }
        }

        return x * y
    }

    override fun partTwo(input: List<Pair<String, Int>>): Any {
        var aim = 0
        var x = 0
        var y = 0

        for(p in input) {
            when(p.first) {
                "forward" -> {
                    x += p.second
                    y += aim * p.second
                }
                "down" -> aim += p.second
                "up" -> aim -= p.second
            }
        }

        return x * y
    }

    override fun adaptInput(input: List<String>): List<Pair<String, Int>> = input.map {
        val s = it.split(' ')
        return@map Pair(s[0], s[1].toInt())
    }
}