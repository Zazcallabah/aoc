$data = (gc 3.txt) -split ""

$locations = @{"0x0y"=1}
$x=0
$y=0

foreach( $move in $data)
{
	if( $move -eq ">" )
	{
		$x++
	}
	elseif( $move -eq "<" )
	{
		$x--
	}
	elseif( $move -eq "^" )
	{
		$y++
	}
	elseif( $move -eq "v" )
	{
		$y--
	}
	$label = "$($x)x$($y)y"
	if(!$locations.ContainsKey($label))
	{
		$locations.Add($label,1)
	}
	else
	{
		$locations[$label]++
	}
}

$locations.keys | measure



$locations = @{"0x0y"=1}
$x=0
$y=0

$x2=0
$y2=0

$i=0
while( $i -lt $data.length)
{
	$move = $data[$i]
	$move2 = $data[$i+1]
	if( $move -eq ">" )
	{
		$x++
	}
	elseif( $move -eq "<" )
	{
		$x--
	}
	elseif( $move -eq "^" )
	{
		$y++
	}
	elseif( $move -eq "v" )
	{
		$y--
	}
	$label = "$($x)x$($y)y"
	if(!$locations.ContainsKey($label))
	{
		$locations.Add($label,1)
	}
	else
	{
		$locations[$label]++
	}

	if( $move2 -eq ">" )
	{
		$x2++
	}
	elseif( $move2 -eq "<" )
	{
		$x2--
	}
	elseif( $move2 -eq "^" )
	{
		$y2++
	}
	elseif( $move2 -eq "v" )
	{
		$y2--
	}
	$label = "$($x2)x$($y2)y"
	if(!$locations.ContainsKey($label))
	{
		$locations.Add($label,1)
	}
	else
	{
		$locations[$label]++
	}
	$i += 2
}

$locations.keys | measure