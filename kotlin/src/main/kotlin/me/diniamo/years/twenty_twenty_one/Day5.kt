package me.diniamo.years.twenty_twenty_one

import me.diniamo.Day
import me.diniamo.rangeOrDownTo
import java.lang.Integer.max

private enum class DiagDir(val x: Int, val y: Int) {
    UP_LEFT(-1, -1),
    UP_RIGHT(1, -1),
    DOWN_LEFT(-1, 1),
    DOWN_RIGHT(1, 1)
}

class Day5(year: Int) : Day<List<Pair<Pair<Int, Int>, Pair<Int, Int>>>>(5, year) {
    private fun generateOcean(input: List<Pair<Pair<Int, Int>, Pair<Int, Int>>>, diagonals: Boolean): Array<Array<Int>> {
        val maxX = max(input.maxOf { it.first.first }, input.maxOf { it.second.first }) + 1
        val maxY = max(input.maxOf { it.first.second }, input.maxOf { it.second.second }) + 1

        val ocean = Array(maxY) { Array(maxX) { 0 } }  // To access a point: ocean[y][x]

        for(p in input) {
            val isDiagonal = p.first.first != p.second.first && p.first.second != p.second.second
            if(!diagonals && isDiagonal)
                continue

            if(isDiagonal) {
                val x1 = p.first.first
                val y1 = p.first.second
                val x2 = p.second.first
                val y2 = p.second.second

                val dir: DiagDir = when {
                    x1 > x2 && y1 > y2 -> DiagDir.UP_LEFT
                    x1 < x2 && y1 > y2 -> DiagDir.UP_RIGHT
                    x1 > x2 && y1 < y2 -> DiagDir.DOWN_LEFT
                    x1 < x2 && y1 < y2 -> DiagDir.DOWN_RIGHT
                    else -> throw IllegalStateException("not how diagonals works, just let me compile this")
                }

                var x = x1
                var y = y1
                while(x != x2 && y != y2) {
                    ocean[y][x]++

                    x += dir.x
                    y += dir.y
                }
                ocean[y][x]++
            } else {
                for (x in (p.first.first rangeOrDownTo p.second.first)) {
                    for (y in (p.first.second rangeOrDownTo p.second.second)) {
                        ocean[y][x]++
                    }
                }
            }
        }

        return ocean
    }

    override fun partOne(input: List<Pair<Pair<Int, Int>, Pair<Int, Int>>>): Any {
        val ocean = generateOcean(input, false)
        return ocean.sumOf { it.count { int -> int > 1 } }
    }

    override fun partTwo(input: List<Pair<Pair<Int, Int>, Pair<Int, Int>>>): Any {
        val ocean = generateOcean(input, true)
        return ocean.sumOf { it.count { int -> int > 1 } }
    }

    override fun adaptInput(input: List<String>): List<Pair<Pair<Int, Int>, Pair<Int, Int>>> = input.map { str ->
        str.split(" -> ").let {
            Pair(it[0].splitToPairInt(","), it[1].splitToPairInt(","))
        }
    }

    private fun String.splitToPairInt(delimiter: String, ignoreCase: Boolean = false):Pair<Int, Int> = this.split(delimiter, ignoreCase = ignoreCase).let { Pair(it[0].toInt(), it[1].toInt()) }
}