const fs = require('fs');

fs.readFile('inputday1.txt', 'utf8', (err, data) => {
    if (err) {console.error(err);return;}
    content = data.split(", ");
    let NESW = [0,0,0,0];
    let currdir = 0;
    for (let i =0; i < content.length; i++) {
        curr = content[i];
        dir = curr[0];
        mov = Number(curr.slice(1,));
        if (dir == 'R') {
            currdir = (currdir + 1) % 4;
        } else {
            currdir = (currdir + 3) % 4;
        }
        NESW[currdir] += mov;
    };
    dif = Math.abs(NESW[0] - NESW[2]) + Math.abs(NESW[1] - NESW[3]);
    console.log('Part 1:')
    console.log(dif);
});

fs.readFile('inputday1.txt', 'utf8', (err, data) => {
    if (err) {console.error(err);return;}
    content = data.split(", ");
    let NESW = [0,0,0,0];
    let currdir = 0;
    let seen = new Set(); 
    for (let i =0; i < content.length; i++) {
        curr = content[i];
        dir = curr[0];
        mov = Number(curr.slice(1,));
        if (dir == 'R') {
            currdir = (currdir + 1) % 4;
        } else {
            currdir = (currdir + 3) % 4;
        }
        for (let j = 0; j < mov; j++) {
            NESW[currdir]++;
            dif = Math.abs(NESW[0] - NESW[2]) + Math.abs(NESW[1] - NESW[3]);
            pos = [NESW[0] - NESW[2], NESW[1] - NESW[3]]
            if (seen.has(JSON.stringify(pos))) {
                console.log('Part 2:')
                console.log(dif)
                return;
            }
            else {
                seen.add(JSON.stringify(pos));
            }
        }
    };
    console.log(dif);
});