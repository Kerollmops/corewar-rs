.name "test"
.comment "I'm just a basic test"

l2:     sti r2, %:live, %1
        and r1, %0, r1

live:   live %1
        zjmp %:live
