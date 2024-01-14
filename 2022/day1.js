const fs = require('fs')
const fn = 'inputday1.txt'

fs.readFile(fn, 'utf8', (err, data) => {
    if (err) {
        console.error(err)
        return
    }

    // Part 1  
    const lines = data.split('\n')
    part1 = 0
    temp = 0
    for (let i = 0; i < lines.length; i++) {
        if (lines[i] != '') {
            temp += parseInt(lines[i])
        }
        else {
            if (temp > part1) {
                part1 = temp
            }
            temp = 0
        }
    }
    console.log(`Part 1: ${part1}`)

    // Part 2
    calories = []
    for (let i = 0; i < lines.length; i++) {
        if (lines[i] != '') {
            temp += parseInt(lines[i])
        }
        else {
            calories.push(temp)
            temp = 0
        }
    }
    calories.sort((a, b) => b - a)
    part2 = calories[0] + calories[1] + calories[2]
    console.log(`Part 2: ${part2}`)
})