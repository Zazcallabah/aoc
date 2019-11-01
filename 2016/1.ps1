$input = "R4, R4, L1, R3, L5, R2, R5, R1, L4, R3, L5, R2, L3, L4, L3, R1, R5, R1, L3, L1, R3, L1, R2, R2, L2, R5, L3, L4, R4, R4, R2, L4, L1, R5, L1, L4, R4, L1, R1, L2, R5, L2, L3, R2, R1, L194, R2, L4, R49, R1, R3, L5, L4, L1, R4, R2, R1, L5, R3, L5, L4, R4, R4, L2, L3, R78, L5, R4, R191, R4, R3, R1, L2, R1, R3, L1, R3, R4, R2, L2, R1, R4, L5, R2, L2, L4, L2, R1, R2, L3, R5, R2, L3, L3, R3, L1, L1, R5, L4, L4, L2, R5, R1, R4, L3, L5, L4, R5, L4, R5, R4, L3, L2, L5, R4, R3, L3, R1, L5, R5, R1, L3, R2, L5, R5, L3, R1, R4, L5, R4, R2, R3, L4, L5, R3, R4, L5, L5, R4, L4, L4, R1, R5, R3, L1, L4, L3, L4, R1, L5, L1, R2, R2, R4, R4, L5, R4, R1, L1, L1, L3, L5, L2, R4, L3, L5, L4, L1, R3"

$instr = $input -split "," | %{ $_.trim() }

$visited = @{}
$script:found = $false

$dir = 0 # north

$longitude = 0
$latitude = 0

function Right
{
	param($n)
	($n + 1) % 4
}

function Left
{
	param($n)
	($n+3) % 4
}

function Distance
{
	param($x,$y)
	[Math]::abs($x)+[Math]::abs($y)
}
function Visit
{
	param($longitude,$latitude)
	$key = "$longitude,$latitude"
	if( $visited.ContainsKey($key) )
	{
		if( !$script:found )
		{
			write-host "double visit $key ($(Distance $longitude $latitude))"
			$script:found = $true
		}
	}
	else
	{
		$visited.Add($key,"");
	}
}
$instr | %{
	if( $_[0] -eq "R" )
	{
		$dir = Right $dir
	}
	else
	{
		$dir = Left $dir
	}
	$distance = [int]$_.substring(1)
	if( $dir -eq 0 ){
		($latitude+1)..($latitude+$distance) | %{ Visit $longitude $_ }
		$latitude += $distance
	}
	if( $dir -eq 1 ){
		($longitude+1)..($longitude+$distance) | %{ Visit $_ $latitude }
		$longitude += $distance
	}
	if( $dir -eq 2 ){
		($latitude-$distance)..($latitude-1) | %{ Visit $longitude $_ }
		$latitude -= $distance
	}
	if( $dir -eq 3 ){
		($longitude-$distance)..($longitude-1) | %{ Visit $_ $latitude }
		$longitude -= $distance
	}


}
write-host "end $longitude,$latitude ($(Distance $longitude $latitude))"
