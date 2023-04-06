
for x in {1..100}
do
    for y in {1..100}
    do
        echo $x $y > input/d
        make d_test > /dev/null 2>&1
        if [ $? -ne 0 ]; then
            echo err in $x $y
        fi
    done
done
