const fs = require('fs')
const fn = 'inputday2.txt'

fs.readFile(fn, 'utf8', (err, data) => {
    if (err) {
        console.error(err)
        return
    }
    // Part 1  
    let part1 = 0
    let lines = data.split('\n');
    lines.forEach(line => {
        moves = line.split(' ')
        //console.log(moves)
        let move_point = moves[1].charCodeAt(0) - 87
        let asciiDifference = (moves[1].charCodeAt(0) - moves[0].charCodeAt(0)) % 3;
        let game_point;
        switch (asciiDifference) {
            case 0:
                game_point = 6;
                break;
            case 1:
                game_point = 0;
                break;
            case 2:
                game_point = 3;
                break;
        }
        part1 += move_point
        part1 += game_point
    });
    console.log(`Part 1: ${part1}`)
    // Part 2
    let part2 = 0
    lines.forEach(line => {
        let moves = line.split(' ')
        let game_point
        switch (moves[1]) {
            case 'X':
                game_point = 0;
                break
            case 'Y':
                game_point = 3;
                break
            case 'Z':
                game_point = 6;
                break
        }
        let move_point
        switch (moves[1]) {
            case 'X':
                move_point = ((moves[0].charCodeAt(0) - 66 +3) % 3) +1
                break
            case 'Y':
                move_point = ((moves[0].charCodeAt(0) - 65 +3) % 3) +1
                break
            case 'Z':
                move_point = ((moves[0].charCodeAt(0) - 64 +3) % 3) +1
                break
        }

        console.log(move_point, game_point)
        part2 += move_point
        part2 += game_point
    });

    console.log(`Part 2: ${part2}`)
})