`malloc` and `free` has to be up there for the functions with the highest implementation complexity to API complexity.

Google, Facebook, and Microsoft (twice!) have all found it worthwhile to write their own implementations of malloc.


While working on a runtime for a programing language [^tycheshow_post] I realized that a signifigant amount of the complexity of a `malloc` implementation wasn't needed for this specific use case. There were infact 2 key simplifications to make:


[^tycheshow_post]: I'll try to write about it at some point when I can, it's got an incredibly cool self hosted compiler.

1. Syncronization: This programming language is single threaded, so all allocations and frees come from the same thread.
2. Sizes: For language releated reasons, the only data structure that need heap allocation is a `cons`-like Pair. More importantly, they're all the same size allocation, and we know that at compile time.

More formally, our data looks like this[^union].

```c
struct PairData {
  struct PairData *fst;
  struct PairData *snd;
};
typedef struct PairData *Pair;
```

[^union]: In the actual code, `fst` and `snd` are union, one of which's members is a pointer to `PairData`, but that's not important here.

## A simple Free List

A free list with two pairs in in would look like so: [^snd]

[^snd]: In all diagrams, the `snd` pointer is ommited for clarity.

```asciiart
  ╔══════╗
  ║ PAIR ║ ◁── free_list
  ╚══╤═══╝
     │
     ▼
  ╔══════╗
  ║ PAIR ║
  ╚══╤═══╝
     │
    ╭▼╮
    │╳│ NULL
    ╰─╯

```



```c
static Pair free_list = NULL;

void rt_free_pair(Pair to_free) {
  to_free->fst = free_list;
  free_list = to_free;
}

Pair rt_alloc_pair() {
  Pair to_alloc = free_list;
  free_list = to_alloc->fst;
  return to_alloc;
}
```

### Freeing by `cons`ing onto the free list.

```asciiart
  ╔══════╗
  ║ PAIR ║◁── to_free
  ╚══╤═══╝
     │
     └────── ▶ wherever


  ╔══════╗
  ║ PAIR ║ ◁── free_list
  ╚══╤═══╝
     │
     ▼
  ╔══════╗
  ║ PAIR ║
  ╚══╤═══╝
     │
    ╭▼╮
    │╳│ NULL
    ╰─╯
```

We make the freed pair point to the head of the free list.

```asciiart
  ╔══════╗
  ║ PAIR ║◁─── to_free
  ╚══╤═══╝
     │
     ▼
  ╔══════╗
  ║ PAIR ║◁─── free_list
  ╚══╤═══╝
     │
     ▼
  ╔══════╗
  ║ PAIR ║
  ╚══╤═══╝
     │
    ╭▼╮
    │╳│ NULL
    ╰─╯
```

Then we update the free_list pointer to be the new element.

```asciiart
  ╔══════╗
  ║ PAIR ║ ◁── to_free
  ╚══╤═══╝ ◁── free_list
     │
     ▼
  ╔══════╗
  ║ PAIR ║ 
  ╚══╤═══╝
     │
     ▼
  ╔══════╗
  ║ PAIR ║
  ╚══╤═══╝
     │
    ╭▼╮
    │╳│ NULL
    ╰─╯
```

And now the pair is in the free list, ready to be used for the next allocation.


### Allocating by poping

```asciiart
  ╔══════╗
  ║ PAIR ║ ◁── free_list
  ╚══╤═══╝
     │
     ▼
  ╔══════╗
  ║ PAIR ║
  ╚══╤═══╝
     │
    ╭▼╮
    │╳│ NULL
    ╰─╯
```
We save the head node as the memory to be returned
```asciiart
  ╔══════╗
  ║ PAIR ║ ◁── free_list
  ╚══╤═══╝ ◁── to_alloc
     │
     ▼
  ╔══════╗
  ║ PAIR ║
  ╚══╤═══╝
     │
    ╭▼╮
    │╳│ NULL
    ╰─╯
```
And advance the free_list pointer along to the next node.

```asciiart
  ╔══════╗
  ║ PAIR ║ ◁── to_alloc
  ╚══╤═══╝ 
     │
     ▼
  ╔══════╗
  ║ PAIR ║ ◁── free_list
  ╚══╤═══╝
     │
    ╭▼╮
    │╳│ NULL
    ╰─╯
```

(while at this point, the returned pair's `fst` still points to the free list, this is never exposed to user application code. It'll get overriden by whatever they specified in the pair's constructor. The runtime just has different functions for pair allocation and initialization.)

### Actually getting memory

while this is all fine and dandy linked-list code, it doesn't *actually get memory* to give out. In the case that the free list is empty, we need to extend it.

```c
Pair rt_alloc_pair() {
  if (free_list == NULL) extend_freelist();
  // --- snip ---
```

To actualy extend the free list, we'll allocate a chuck of memory from the underlying system (with `malloc`, of course), and then link them all together onto the free list.

```c
static void extend_freelist() {
  const size_t CHUNKSIZE = 4096;
  Pair chunk = malloc(CHUNKSIZE * sizeof(struct PairData));
  for (size_t i = 0; i < CHUNKSIZE; i++) {
    chunk[i].fst = free_list;
    free_list = &chunk[i];
  }
}
```

```asciiart
                       CHUNKSIZE  pairs
                ╭┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄╮  
          ╔══════╦══════╦══════╦══════╦══════╦══════╗
          ║ PAIR ║ PAIR ║ PAIR ║ PAIR ║ PAIR ║ PAIR ║◁── free_list
     ╭─╮  ╚╤═══▲═╩═╤══▲═╩═╤══▲═╩═╤══▲═╩═╤══▲═╩═╤════╝
NULL │╳◄───┘   └───┘  └───┘  └───┘  └───┘  └───┘
     ╰─╯        
```
