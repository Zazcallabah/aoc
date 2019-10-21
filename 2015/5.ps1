$data = gc 5.txt

$data | ?{

	if( $_ -match "ab|cd|pq|xy" )
	{
		return $false
	}

	if( $_ -match "[aoeui].*[aoeui].*[aoeui]" )
	{
		#97..(97+25) | %{ write-host -nonewline "$([char]$_)$([char]$_)|" }
		if( $_ -match "aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz" )
		{
			return $true
		}
	}
	return $false

} | measure

function hasInterRepeat
{
	param($str)

	$i = 0
	while($true)
	{
		if( $i+2 -ge $str.length )
		{
			return $false
		}
		$c1 = $str[$i]
		$c2 = $str[$i+1]
		$c3 = $str[$i+2]

		if($c1 -eq $c3 -and $c1 -ne $c2 )
		{
			return $true
		}
		$i++
	}
}

function hasDupe
{
	param($str)

	$i = 0
	while($true)
	{
		if( $i+2 -ge $str.length )
		{
			return $false
		}
		$c1 = $str[$i]
		$c2 = $str[$i+1]

		if( $str.indexOf("$c1$c2",$i+2) -ne -1 )
		{
			return $true
		}
		$i++
	}
}

$data | ?{

	if( hasInterRepeat $_ )
	{
		if( hasDupe $_ )
		{
			return $true
		}
	}
	return $false


} | measure


