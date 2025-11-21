package me.diniamo.years.twenty_twenty

import me.diniamo.*


class Day4(year: Int) : Day<List<String>>(4, year) {
    private val requiredInfo = listOf(
        "ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"
    )

    private val spaceOrNewlineRegex = Regex("\\n| ")

    override fun partOne(input: List<String>): Any {
        var valid = 0

        for (l in input) {
            if (l == "\n") continue

            val parsed = l.split(spaceOrNewlineRegex).filter { it.isNotEmpty() }.map { it.substringBefore(":") }

            if (
                parsed.containsAll(requiredInfo)
            ) valid++
        }

        return valid
    }

    private val checkMap = mapOf<String, (String) -> Boolean>(
        "byr" to { it.toIntOrNull() in 1920..2002 },
        "iyr" to { it.toIntOrNull() in 2010..2020 },
        "eyr" to { it.toIntOrNull() in 2020..2030 },
        "hgt" to {
            when {
                it.endsWith("cm") -> it.dropLast(2).toIntOrNull()?.takeIf { n -> n in 150..193 } != null
                it.endsWith("in") -> it.dropLast(2).toIntOrNull()?.takeIf { n -> n in 59..76 } != null
                else -> false
            }
        },
        "hcl" to { it.matches(Regex("^#[0-9a-f]{6}$")) },
        "ecl" to { arrayOf("amb", "blu", "brn", "gry", "grn", "hzl", "oth").contains(it) },
        "pid" to { it.length == 9 && it.all(Char::isDigit) },
    )

    override fun partTwo(input: List<String>): Any {
        var valid = 0

        input.forEachIndexed { i, l ->
            val parsed = l.split(spaceOrNewlineRegex).filter { it.isNotEmpty() }
                .map { it.split(':').let { split -> split[0] to split[1] } }.toMutableList()

            for (j in parsed.indices) {
                if (parsed[j].first == "cid") {
                    parsed.removeAt(j)
                    break
                }
            }

            if (
                parsed.filter {requiredInfo.contains(it.first)}.toSet().size == requiredInfo.size &&
                parsed.all {
                    checkMap[it.first]?.invoke(it.second)
                        .also { b -> if(b!!) printColored(ANSI_GREEN, "${it.first + ":" + it.second} is valid.") else printColored(ANSI_RED, "${it.first + ":" + it.second} is invalid.") }
                        ?: error("Day 4 check invoke is fucked")
                }.also { printColored(ANSI_CYAN, "Passport $i is ${if(it) "valid" else "invalid"}.") }
            ) {
                valid++
            }
            println()
        }

        return valid
    }

    private val blankLineRegex = Regex("\r?\n\\s*\r?\n")
    override fun adaptInput(input: List<String>) = input.let {
        val list = mutableListOf<String>()

        list.addAll(input.joinToString("\n").split(blankLineRegex))

        list
    }
}