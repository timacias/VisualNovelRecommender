# Visual Novel Recommender
COP3530 Final Project

**How to run program:**
NEEDED:
- *Rust* 
<ol> If using VSCode, use the rust-analyzer extension. You can  also use an IDE with Rust like RustRover </ol>

- *npm (Node.js)* 
<ol> </ol>

- *React*
    > Type `npm install react` into your terminal

---
**Steps**
<ol>
    <li> Open up terminal and go to “my_rust_backend” </li>
    <li> Run with `cargo run`. Note this WILL take a long time as it is loading the database and calculating edge weights for every novel, approximately 10-20 minutes. Once you see “listening on 127.0.0.1” in your terminal, that means that the backend has created the graph and is ready for usage. </li>
    <li> Open a new terminal and move to the “my_react_frontend” folder </li>
    <li>Type `PORT=3001 npm start` in the new terminal
        <ol>
          <li> If this does not work, do `npm run build` </li>
          <li> Then `serve -s build` </li>
        </ol>
    </li>
</ol>

---
**Disclaimers:**

The database is from June 24th, 2024. Details of the novels such as tags are added by users so newer novels may not be tagged enough, hence some obvious NSFW novels may pass into our database because they have not been tagged properly for the database to recognize it as NSFW.
Any novel with a tag categorized as ‘ero’ was classified as NSFW.

Some images/titles that are displayed may not be right, but it is due to the Visual Novel API inconsistencies with its own search function.
<ul>
  <li>For example, inputting Fate/Extra CCC returns from the API and displays its prequel Fate/Extra but our backend is working with Fate/Extra CCC.
</li>
  <li>These titles are usually a part of the same series. Our database and algorithms find the novels that the user has inputted correctly.
</li>

</ul>
Please avoid using novels of the same series or novels super similar to each other as they might just return a path with just the two novels.

Our program only displays the top 3 tags in a novel for visual clarity, but we use all the tags in out calculations.

Graph edges are based on similarity score (see ‘Extra Details’) we only create an edge if a certain similarity percentage is met.

On some very rare occasions, using the other algorithm will result in a different path.

---

**Extra Details:**

Weight Calculations

Each novel has three attributes which are sets: staff, seiyuu (voice actor), and tags. When compared, the cardinality of the intersection of each set is found and added an “intersection_index”. Afterward, each attribute is compared to find which set is smaller and is added to “smallest_sizes”. This is so we can find a percentage of similarities found within the limitations of the unknown variation between quantities of each attribute’s set.

We find the percentage similarity by doing intersection_index/smallest_sizes * 100.0. Finally, since we are doing the shortest path algorithms, we have to subtract the percentage by 100 so that in these algorithms, the more similar novels are considered before others.


