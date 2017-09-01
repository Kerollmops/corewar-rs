.name "zork"
.comment "I'M ALIIIIVE"
.extend

l2:		sti r1, %:live, %1 # test
		and r1,%0,r1#test2

live:	live %1
		zjmp %:live
