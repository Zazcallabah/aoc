$data = gc 2.txt

#29x13x26
#11x11x14
#...
$data | %{
	$n = $_ -split "x" | %{ [int]$_ } | sort
	$smallest = $n[0]*$n[1]
	$mid = $n[0]*$n[2]
	$large = $n[1]*$n[2]
	if( $smallest -gt $mid -or $smallest -gt $large )
	{
		throw "bad assumption"
	}
	$area = 2*($smallest+$mid+$large)
	return $area+$smallest
} | measure -sum


$data | %{
	$n = $_ -split "x" | %{ [int]$_ } | sort

	$volume = $n[0]*$n[1]*$n[2]

	$length = 2*($n[0]+$n[1])
	return $volume+$length
} | measure -sum