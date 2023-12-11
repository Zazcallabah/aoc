$data = gc "$psscriptroot/11.txt"

function sumall {
	param($arr)
	$sum = 0

	for($outer=0;$outer -lt $arr.length-1; $outer++){
		for($inner=$outer+1; $inner -lt $arr.length; $inner++){
			$o = $arr[$outer]
			$i = $arr[$inner]

			$sum += [math]::abs($o.r-$i.r)
			$sum += [math]::abs($o.c-$i.c)
		}
	}

	$sum
}
function sumByMap {
	param($arr,$map)
	$sum = 0

	for($outer=0;$outer -lt $arr.length-1; $outer++){
		for($inner=$outer+1; $inner -lt $arr.length; $inner++){
			$o = $arr[$outer]
			$i = $arr[$inner]

			($o.r)..($i.r) | %{
				$sum += $map.rows[$_]
			}
			($o.c)..($i.c) | %{
				$sum += $map.cols[$_]
			}
			$sum -= 2
		}
	}

	$sum
}


function parse {
	param($str)
	$allcoords=@()
	$lines = $str -split "`n"
	$width = $lines[0].length
	for($row=0;$row -lt $lines.length; $row++){
		for($col=0; $col -lt $width; $col++){
			if( $lines[$row][$col] -eq "#" ){
				$allcoords += @{"r"=$row;"c"=$col}
			}
		}
	}
	$allcoords
}

function blankcolumn
{
	param($arr, $ix)
	for($i=0; $i -lt $arr.length; $i++)
	{
		if($arr[$i][$ix] -ne ".")
		{
			return $false;
		}
	}
	return $true

}

function growth
{
	param($str)
	$lines = $str -split "`n"
	$result = @()
	$lines | %{
		$result += $_
		if( $_ -match "^\.+$" )
		{
			$result += $_
		}
	}

	$width = $result[0].length
	$height = $result.length
	$result2=1..$height | %{ "" }

	for($column=0; $column -lt $width; $column++)
	{
		$double = blankcolumn $result $column
		for($row=0; $row -lt $result.length; $row++)
		{
			$cc = $result[$row][$column]
			$result2[$row] += $cc
			if($double)
			{
				$result2[$row] += $cc
			}
		}
	}
	return $result2 -join "`n"
}
function weightmaps {
	param($str,$weight)
	$lines = $str -split "`n"
	$rows = @()
	$lines | %{
		if( $_ -match "^\.+$" ){
			$rows += $weight
		} else {
			$rows += 1
		}
	}

	$width = $lines[0].length
	$cols=@()

	for($column=0; $column -lt $width; $column++)
	{
		$double = blankcolumn $lines $column
		if($double){
			$cols += $weight
		} else {
			$cols += 1
		}
	}

	return @{"rows"=$rows;"cols"=$cols}
}
describe "testdata" {
	$test="...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."

	it "grows" {
		growth "." | should be "..`n.."
		growth $test | should be "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."

	}
	it "can parse" {
		$result = parse "#.`n.#"
		$result.length | should be 2
		$result[0].r | should be 0
		$result[1].c | should be 1
	}
	it "can sum" {
		$result = growth $test
		$parsed = parse $result
		sumall $parsed | should be 374
	}
	it "can get weightmap" {
		$result = weightmaps $test 2
		$result.cols | should be @(1,1,2,1,1,2,1,1,2,1)
	}
	it "can sum by weightmap" {
		$result = weightmaps $test 2
		$parsed = parse $test
		sumByMap $parsed $result | should be 374
	}
}

$result = growth $data
$parsed = parse $result
$summed = sumall $parsed

write-host "summed: $summed"


$result = weightmaps $data 1000000
$parsed = parse $data
$summed = sumByMap $parsed $result

write-host "expanded: $summed"

