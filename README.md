# conway
Implementation of Conway's Game of  Life

## Usage
```bash
conway -f <FILE> -i <ITERATIONS> -s <SIZE> -o <OUTPUT>
```

### File

File describing the initial state. Here is an example :
```
000000
011010
011000
000110
000110
```
If not set, the program will look for a file named `conway.txt` in the current path.

### Iterations

Number of iterations drawn into the resulting gif.

### Size

Size of the cells. For example, with the above file and <SIZE> = 50, the resulting gif will have a size of 300px by 300px.

### Output

File where the frames are drawn. If not set, the program will write in a file named `conway.gif` in the current path.

## Current result
![](conway.gif)