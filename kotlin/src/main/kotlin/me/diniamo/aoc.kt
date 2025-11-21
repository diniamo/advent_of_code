@file:JvmName("AOC")
package me.diniamo

import me.diniamo.years.twenty_twenty_one.Aoc2021
import java.io.File
import java.net.HttpURLConnection
import java.net.URL
import java.nio.file.Files
import java.nio.file.Path
import kotlin.system.exitProcess

fun main() {
    val sessionIdFile = File("session_id.txt")
    if(!sessionIdFile.exists()) {
        print("session_id.txt doesn't exist, the program will terminate")
        exitProcess(1)
    }

    val sessionId = sessionIdFile.readText()

    arrayOf(
        //Aoc2020(),
        Aoc2021()
    ).forEach {
        it.evaluate(sessionId)
    }
}

abstract class Year(val year: Int) {
    private val folder = Path.of("$year.txt").also {
        if (!Files.exists(it)) Files.createDirectories(it)
    }

    abstract val days: Collection<Day<out Any>>

    fun evaluate(token: String) {
        println("Year $year:")
        days.forEach { it as Day<Any>
            val rawInput = getInput(it.day, token)

            var startTime = System.currentTimeMillis()
            val input = it.adaptInput(rawInput)
            val inputTime = System.currentTimeMillis() - startTime
            startTime = System.currentTimeMillis()

            println("  Day ${it.day}:")
            println("    Part one solution: " +  it.partOne(input) + "\n    Finished in: ${System.currentTimeMillis() - startTime + inputTime}ms")
            startTime = System.currentTimeMillis()
            println("    Part two solution: " + it.partTwo(input) + "\n    Finished in: ${System.currentTimeMillis() - startTime + inputTime}ms\n")
        }
    }

    private fun getInput(day: Int, token: String): List<String> {
            val file = folder.resolve("$day.txt")
            if (!Files.exists(file)) {
                (URL("https://adventofcode.com/$year/day/$day/input").openConnection() as HttpURLConnection).also { urlConnection ->
                    urlConnection.setRequestProperty("Cookie", "session=$token")
                    urlConnection.doInput = true
                    urlConnection.connect()
                    urlConnection.inputStream.use { Files.copy(it, file) }
                    urlConnection.disconnect()
                }
            }
            return Files.readAllLines(file)
    }
}

abstract class Day<T>(val day: Int, year: Int) {
    abstract fun partOne(input: T): Any
    abstract fun partTwo(input: T): Any

    abstract fun adaptInput(input: List<String>): T

    private val file: Path = Path.of("inputs/$year/$day.txt")
}