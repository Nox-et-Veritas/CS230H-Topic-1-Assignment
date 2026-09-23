- Project Pitch: I want to create a system to track users I want to attack in the browser game Torn
	- linked data structure: a linked list (or something similar) of targets, so I can rapidly hit targets one after another
	- multiple concurrent control flows: system will be able to supply targets to multiple end users of the system, and use different API keys at once
	- network: do calculations on the server/use Torn's API and serve list of feasible/relevant targets to end users

Attempting to use the variable that I passed ownership of to another function returned an error when trying to compile for a non-Copy type variable (String).

![[Pasted image 20260923185252.png]]

Meanwhile, doing the same for an i32 which has the Copy attribute did not cause any errors upon compilation.
![[Pasted image 20260923184938.png]]