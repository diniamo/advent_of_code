package me.diniamo.years.twenty_twenty_one

import me.diniamo.Day

class Day6(year: Int) : Day<List<Int>>(6, year) {
    private fun countLanternFish(start: List<Int>, amountOfDays: Int): Long {
        val map = hashMapOf<Int, Long>(
            0 to 0,
            1 to 0,
            2 to 0,
            3 to 0,
            4 to 0,
            5 to 0,
            6 to 0,
            7 to 0,
            8 to 0,
        )

        for(timer in start) {
            map[timer] = map[timer]!!.plus(1)
        }

        for(day in 1..amountOfDays) {
            val oz = map[0]!!

            map[0] = map[1]!!
            map[1] = map[2]!!
            map[2] = map[3]!!
            map[3] = map[4]!!
            map[4] = map[5]!!
            map[5] = map[6]!!
            map[6] = map[7]!!
            map[7] = map[8]!!

            map[6] = map[6]!!.plus(oz)
            map[8] = oz
        }

        return (0..8).sumOf { map[it]!! }
    }

    override fun partOne(input: List<Int>): Any = countLanternFish(input, 80)
    override fun partTwo(input: List<Int>): Any = countLanternFish(input, 256)

    override fun adaptInput(input: List<String>): List<Int> = input[0].split(',').map { it.toInt() }
}