<?php
   $i = 0;
    while ($i < 3) {
        echo "-- Loop when i = $i\n";
        $i++;
        while (true) {
            while (true) {
                echo "This is the inner loop\n";
                continue 3;
            }
            echo "This is the loop in the middle\n";
        }
    }
    
    echo "\n";
    
    for ($j = 0; $j < 3; $j++) {
        echo "-- Loop when j = $j\n";
        while(true) {
            break;
            echo "The inner loop\n";    
        }
    }
    
    echo "\n";
    // What happens when break is used with number
    for ($j = 0; $j < 3; $j++) {
        echo "-- Loop when j = $j\n";
        while(true) {
            break 2;
        }
    }
?>
