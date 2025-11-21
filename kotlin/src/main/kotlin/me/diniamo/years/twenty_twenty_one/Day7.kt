package me.diniamo.years.twenty_twenty_one

import me.diniamo.Day
import kotlin.math.abs

class Day7(year: Int) : Day<List<Int>>(7, year) {
    private fun countFuelCostSingle(toPos: Int, fromPositions: List<Int>): Int = fromPositions.sumOf { abs(toPos - it) }
    override fun partOne(input: List<Int>): Any = (input.minOrNull()!!..input.maxOrNull()!!).minOf { countFuelCostSingle(it, input) }

    private fun countFuelCostDoubling(toPos: Int, fromPositions: List<Int>): Int = fromPositions.sumOf { pos -> (1..abs(toPos - pos)).sum() }
    override fun partTwo(input: List<Int>): Any = (input.minOrNull()!!..input.maxOrNull()!!).minOf { countFuelCostDoubling(it, input) }

    override fun adaptInput(input: List<String>): List<Int> = input[0].split(',').map { it.toInt() }
}