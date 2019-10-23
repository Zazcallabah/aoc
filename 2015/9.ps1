$data = "Faerun to Tristram = 65
Faerun to Tambi = 129
Faerun to Norrath = 144
Faerun to Snowdin = 71
Faerun to Straylight = 137
Faerun to AlphaCentauri = 3
Faerun to Arbre = 149
Tristram to Tambi = 63
Tristram to Norrath = 4
Tristram to Snowdin = 105
Tristram to Straylight = 125
Tristram to AlphaCentauri = 55
Tristram to Arbre = 14
Tambi to Norrath = 68
Tambi to Snowdin = 52
Tambi to Straylight = 65
Tambi to AlphaCentauri = 22
Tambi to Arbre = 143
Norrath to Snowdin = 8
Norrath to Straylight = 23
Norrath to AlphaCentauri = 136
Norrath to Arbre = 115
Snowdin to Straylight = 101
Snowdin to AlphaCentauri = 84
Snowdin to Arbre = 96
Straylight to AlphaCentauri = 107
Straylight to Arbre = 14
AlphaCentauri to Arbre = 46" -split "`n"

$map = @{}
$r = [regex]"(\w+) to (\w+) = (\d+)"
$data | %{
	$match = $r.Match( $_ )

	$from = $match.Groups[1].Value
	$to = $match.Groups[2].Value
	$dist = $match.Groups[3].Value

	if( !$map.containskey($from) )
	{
		$map.add($from,@{})
	}
	if( !$map.containskey($to) )
	{
		$map.add($to,@{})
	}

	$map[$from].add($to,$dist)
	$map[$to].add($from,$dist)
}

$maximumtravel = $map.GetEnumerator() | %{ $_.Value.GetEnumerator() | %{ $_.Value}  } | measure -max | select -expandproperty maximum
$nodecount = $map.keys | measure | select -expandproperty count
$script:besteffort = $maximumtravel*$nodecount

function Travel
{
	param($from,$to,$traveldistance,$past,$remaining,$map,[switch]$longest)

	if( $from -ne $null )
	{
		$newdistance = $traveldistance + $map[$from][$to]
	}
	else
	{
		$newdistance = 0
	}

	if( !$longest -and $newdistance -ge $script:besteffort )
	{
		return $null
	}

	$newpast = $past+$to

	if($remaining -eq $null)
	{
		if($longest -and $newdistance -le $script:besteffort )
		{
			return $null
		}
		$script:besteffort = $newdistance
		return $newpast
	}

	$best = $null

	for($i=0; $i -lt $remaining.Length; $i++)
	{
		$head,$tail = $remaining
		$result = Travel $to $head $newdistance $newpast $tail $map -longest:$longest
		if( $result -ne $null )
		{
			$best = $result
		}
		if( $tail -ne $null )
		{
			if( $tail.GetType().Name -eq "String" )
			{
				$remaining = ,$tail+$head
			}
			else
			{
				$remaining = $tail+$head
			}
		}
	}
	return $best
}

function StartTravel
{
	param( $map, [switch]$longest )

	$targets = $map.Keys.Clone()

	$best = $null
	$targets | %{
		$first = $_
		$result = Travel $null $_ 0 @() ($targets|?{$_ -ne $first}) $map -longest:$longest
		if( $result -ne $null )
		{
			$best = $result
		}
	}

	return $best
}


$result = StartTravel $map
Write-Host $result
Write-Host $script:besteffort

$script:besteffort = 0
$result = StartTravel $map -longest
Write-Host $result
Write-Host $script:besteffort

Describe "Travel" {

	$m = @{
		"a"=@{"b"=2;"c"=3;};
		"b"=@{"a"=2;"c"=5;};
		"c"=@{"b"=5;"a"=3;};
	}

	It "handles starttravel" {
		$script:besteffort=1000;
		StartTravel $m | should be @("c","a","b")
		$script:besteffort | should be 5
	}

	It "handles two step" {
		$script:besteffort=1000;
		Travel "c" "b" 0 @("c") @("a") $m | should be @("c","b","a")
		$script:besteffort | should be 7
	}
	It "handles three step" {
		$script:besteffort=1000;
		Travel $null "a" 0 @() @("b","c") $m | should be @("a","b","c")
		$script:besteffort | should be 7
	}
	It "handles last step" {
		$script:besteffort=1000;
		Travel "a" "b" 3 @("c","a") $null $m | should be @("c","a","b")
		$script:besteffort | should be 5
	}
}
