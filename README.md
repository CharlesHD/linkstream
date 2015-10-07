Linkstream is a tool for calculating on linkstreams.

# Installation
The tool is written in rust so you'll need [installing rust utilities](https://www.rust-lang.org/downloads.html).
Then just do :
```bash
make
sudo make install
```


# Usage
The tool reads linkstreams on standard input and expects some format and properties :
## Linkstreams format
A linkstream is a succession of link.
A link is written ```n1 n2 t``` where n1 and n2 are numbers representing nodes, and t a number representing the time the link is performed.
A linkstream is written one link per line :
```
2 1 329012
0 1 328274
3 4 328274
```
*The tool expects the linkstream provided being sorted by decreasing time and the nodes being named from 0 to the total number of distinct nodes.*

## linkstream calc connexity <delta> <nbNodes>
You have to provide to this command a delta and the numbers of nodes in the provided linkstream.
This command outputs lines of the form ```time bool```. One of these lines means that the stream provided on stdin is connected between [time, time + delta] if and only if ```bool``` is ```true```.


## linkstream calc comps [up] <delta> <nbNodes>
This command approximate the delta-connected components of the provided linkstream.
The output is a line of the form ```nc max tab``` where :
* nc is the numbers of really connected components calculated
* max is the size of the maximum component
* tab is the list of the calculated components, the nc first are really connected, the following aren't.

There is two approximation for delta-connected components, one is an upperbound, the other is a lowerbound. By default the lowerbound is used but you can specified the upper by adding the optional keyword ```up``` as : linkstream calc comps up <delta> <nbNodes>

Examples :
```
linkstream calc comps 1000 62 < rollernet.dyn # <- return the lowerbound
linkstream calc comps up 1000 62 < rollernet.dyn # <- return the upperbound


## linkstream calc exist [lr | cut] <delta> <nbNodes>
This command acts differently if provided ```lr```, ```cut``` or nothing.

### linkstream calc exist <delta> <nbNodes>
With no command specifier, the command outputs the delta-existence matrix of the provided linkstream. The xaxis of the matrix is the time and the yaxis is the nodes.

### linkstream calc exist cut <delta> <nbNodes>
With the cut specifier, the command calculates the constant existence time intervals. It outputs lines formatted as follow : ```start stop n...``` where
* start is the start time of the interval
* stop is the stop time of the interval
* each other column are nodes delta-existing in this interval

### linkstream calc exist lr <delta> <nbNodes>
With the lr specifier, the command calculates the largest existing rectangle in the delta-existence matrix. It outputs a line formatted as follow : ```start stop area nbNodes nodes``` where :
* start is the start time of the rectangle
* stop is the stop time of the rectangle
* area is the area of the rectangle
* nbNodes is the height of the rectangles
* nodes is the list of nodes present in the rectangle.

## linkstream calc part [up] <delta> <nbNodes>
This command performs the partitionning of the provided linkstream by existence and by components.
It outputs lines formatted as follow : ```start stop nc max nodes``` where start and stop are the same as in ```linkstream calc exist cut``` and nc max nodes the same as in ```linkstream calc comps```.
As for comps you can specify if you want to use an upperbound algorithm for components with ```up```.

## other commands
calc commands are the main commands of the tool, but it also provide utilities command for manipulating linkstream.

### linkstream rename
Rename the nodes of the provided linkstream for being sure that every nodes between 0 and the maximal node exists.
Outputs a new linkstream.

### linkstream filter (node <node>... | time <start> <stop> | both <start> <stop> <node>...)
Filter the provided linkstream, keeping only the provided nodes or range of time or both.
Outputs a new linkstream

### linkstream info (count (node | links) | degrees <nbNodes> | repart <nbNodes)
* count : counts the number of nodes or links in the provided stream.
* degrees : counts the degree of each node in the provided stream.
* repart : calculates the first and last apparition time for each node in the provided stream.

### linkstream gen <nbNodes> <stop> <proba>
Outputs a randomly generated linkstream with nbNodes nodes and between time 0 and stop.
The linkstream is uniformly generated among all possible links.
