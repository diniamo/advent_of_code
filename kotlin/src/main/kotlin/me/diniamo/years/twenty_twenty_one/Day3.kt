package me.diniamo.years.twenty_twenty_one

import me.diniamo.Day

class Day3(year: Int) : Day<List<String>>(3, year) {
    override fun partOne(input: List<String>): Any {
        val aInput = mutableListOf<String>()
        val strBuilder = StringBuilder()

        for(j in 0 until input[0].length) {
            for(str in input) {
                strBuilder.append(str[j])
            }

            aInput.add(strBuilder.toString())
            strBuilder.clear()
        }

        val gammaBuilder = StringBuilder()
        val epsilonBuilder = StringBuilder()

        aInput.forEach {
            val zeroCount = it.count { c -> c == '0' }
            val oneCount = it.count { c -> c == '1' }

            if(oneCount > zeroCount) {
                gammaBuilder.append('1')
                epsilonBuilder.append('0')
            } else {
                gammaBuilder.append('0')
                epsilonBuilder.append('1')
            }
        }

        return gammaBuilder.toString().toInt(2) * epsilonBuilder.toString().toInt(2)  // .toInt(2) -> convert binary string to decimal
    }


    private fun partTwoRecursive(input: List<String>, index: Int, isLeast: Boolean): List<String> {
        if(input.size == 1)
            return input

        val oneCount = input.count { it[index] == '1' }
        val zeroCount = input.count { it[index] == '0' }

        return if(oneCount > zeroCount || oneCount == zeroCount) {
            partTwoRecursive(
                if (isLeast) input.filter { it[index] == '0' } else input.filter { it[index] == '1' },
                index + 1,
                isLeast
            )
        } else {
            partTwoRecursive(
                if (isLeast) input.filter { it[index] == '1' } else input.filter { it[index] == '0' },
                index + 1,
                isLeast
            )
        }
    }

    override fun partTwo(input: List<String>): Any =
        partTwoRecursive(input, 0, false)[0].toInt(2) * partTwoRecursive(input, 0, true)[0].toInt(2)

    override fun adaptInput(input: List<String>): List<String> = input
}