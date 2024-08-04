# Visual Novel Recommender
COP3530 Final Project

## How to run program
**Dependencies**:
- *Rust* 
	- If using VSCode, use the rust-analyzer extension. Any rust-compatible IDE will work (e.g. RustRover).
	- A standalone install of rust will also work fine (no IDE needed)

- *npm (Node.js)* 

- *React*
	- Type `npm install react` into your terminal

---
**Steps**
1. Open up terminal and go to “my_rust_backend”
2. Run with `cargo run --release`. This will take a long time as it is loading the database and calculating edge weights for every novel. Once you see “listening on 127.0.0.1” in your terminal, that means that the backend has created the graph and is ready for usage.
	- To build a debug binary, use `cargo run`, this will be significantly slower.
3. Open a new terminal and move to the “my_react_frontend” folder
4. Type `PORT=3001 npm start` in the new terminal
	- If this does not work, do `npm run build`
	- Then `serve -s build`

---
## Disclaimers

The database is from June 24th, 2024. Details of the novels such as tags are added by users so some novels (especially newer ones) may have incomplete information. As a result, some NSFW novels may pass into our database because they have not been tagged properly for the database to recognize it as NSFW.

Any novel with a tag categorized as ‘ero’ was classified as NSFW.

Our program only displays the top 3 tags in a novel for visual clarity, but we use all of a novel's tags in our weight and path-finding calculations.

Graph edges are based on similarity score (see 'Extra Details'). An edge will only be created in the graph of novels if a certain similarity percentage is met.

On some very rare occasions, the Dijkstra and Bellman-Ford algorithms will result in different paths. This is expected when multiple valid paths are possible with the same distance/weight traveled.

---

## Extra Details

Weight Calculations:
- Each novel has three attributes which are sets: staff, seiyuu (voice actor), and tags. When compared to another novel, the cardinality of the intersection of each set is found and added to an “intersection_index”. Afterward, each attribute is compared to find which set is smaller and is added to “smallest_sizes”. This is so we can get a percentage of similarities found within the limitations of the variation between the size of each attribute’s set.
- We find the percentage similarity by doing intersection_index/smallest_sizes * 100.0. Finally, since we are doing the shortest path algorithms, we have to subtract the percentage by 100 so that in these algorithms, the more similar novels are considered before others.
