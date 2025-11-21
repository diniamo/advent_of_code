package me.diniamo.years.twenty_twenty

import me.diniamo.Day

class Day2(year: Int) : Day<List<String>>(2, year) {
    override fun partOne(input: List<String>): Any {
        var validPasswords = 0

        for(s in input) {
            val split = s.split(": ")
            val afterHyphen = split[0].substringAfter("-")
            val min = split[0].substringBefore("-").toInt()
            val max = afterHyphen.substringBefore(" ").toInt()
            val char = afterHyphen.substringAfter(" ")[0]

            val charCount = split[1].count { it == char }
            if(charCount in min..max) validPasswords++
        }

        return validPasswords
    }

    override fun partTwo(input: List<String>): Any {
        var validPasswords = 0

        for(s in input) {
            val split = s.split(": ")
            val afterHyphen = split[0].substringAfter("-")
            val first = split[0].substringBefore("-").toInt()
            val second = afterHyphen.substringBefore(" ").toInt()
            val char = afterHyphen.substringAfter(" ")[0]

            if((split[1][first - 1] == char) xor (split[1][second - 1] == char)) validPasswords++
        }

        return validPasswords
    }

    override fun adaptInput(input: List<String>): List<String> = input
}