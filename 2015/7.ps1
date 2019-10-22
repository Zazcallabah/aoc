$script:parser=[regex]"^((?<val>\d+)|(?<link>[a-z]+)|(?<op>NOT) (?<source>.*)|(?<source>\w+) (?<op>AND|OR|LSHIFT|RSHIFT) (?<source2>\w+)) -> (?<id>.*)$"
function Extract
{
	param($str)

	$m = $script:parser.Match($str)
	if(!$m.Success)
	{
		throw "Bad regex on $str"
	}

	$id = $m.Groups["id"].Value
	$op = $null
	$signal = $null
	$source = $null
	$source2 = $null
	if( $m.Groups["val"].Success )
	{
		$op = "VALUE"
		$signal = [uint]$m.Groups["val"].Value
	}
	elseif( $m.Groups["link"].Success )
	{
		$source = $m.Groups["link"].Value
		$op = "LINK"
	}
	elseif( $m.Groups["op"].Success )
	{
		$op = $m.Groups["op"].Value
		$source = $m.Groups["source"].Value
		if( $m.Groups["source2"].Success )
		{
			$source2 = $m.Groups["source2"].Value
		}
	}
	new-object psobject -Property @{"id"=$id;"op"=$op;"signal"=$signal;"source"=$source;"source2"=$source2}
}

function Lookup
{
	param($id,$map)

	if( !$map.ContainsKey($id) )
	{
		throw "shit be bad, yo. $id"
	}
	$obj = $map[$id]

	if( $obj.signal -ne $null )
	{
		return $obj.signal
	}

	if( $obj.source -match "^[0-9]+$" )
	{
		$source = [uint]$obj.source
	}
	else
	{
		$source = Lookup $obj.source $map
	}

	if( $obj.op -eq "LINK" )
	{
		$obj.signal = $source
		return $obj.signal
	}
	if( $obj.op -eq "NOT" )
	{
		$obj.signal = (-bnot [uint]$source -band 0xffff)
		return $obj.signal
	}

	if( $obj.source2 -match "^[0-9]+$" )
	{
		$source2 = [uint]$obj.source2
	}
	else
	{
		$source2 = Lookup $obj.source2 $map
	}

	if( $obj.op -eq "LSHIFT" )
	{
		$obj.signal = ($source -shl $obj.source2 -band 0xffff)
		return $obj.signal
	}
	if( $obj.op -eq "RSHIFT" )
	{
		$obj.signal = ($source -shr $obj.source2 -band 0xffff)
		return $obj.signal
	}
	if( $obj.op -eq "AND" )
	{
		$obj.signal = ($source -band $source2)
		return $obj.signal
	}
	if( $obj.op -eq "OR" )
	{
		$obj.signal = ($source -bor $source2)
		return $obj.signal
	}
}
function BuildMap
{
	param($data)
	$map = @{}
	$data | %{
		$obj = Extract $_
		$map.Add($obj.id,$obj)
	}
	return $map
}

$map = BuildMap (gc 7.txt)

$result = Lookup "a" $map
Write-Host $result

$map = BuildMap (gc 7.txt)
$map["b"].signal = $result
$result = Lookup "a" $map
Write-Host $result


function Validate
{
	Describe "map" {

$data = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i" -split "`n"

		$map = BuildMap $data

		It "d" {
			Lookup "d" $map | should be 72
		}
		It "e" {
			Lookup "e" $map | should be 507
		}
		It "f" {
			Lookup "f" $map | should be 492
		}
		It "g" {
			Lookup "g" $map | should be 114
		}
		It "h" {
			Lookup "h" $map | should be 65412
		}
		It "i" {
			Lookup "i" $map | should be 65079
		}
		It "x" {
			Lookup "x" $map | should be 123
		}
		It "y" {
			Lookup "y" $map | should be 456
		}
	}

	Describe "all ops" {
		$data = gc 7.txt

		$data | %{

			$obj = Extract $_

			$id = $obj.id
			$op = $obj.op
			$signal = $obj.signal
			$source = $obj.source
			$source2 = $obj.source2
			if( $op -eq "VALUE" )
			{
				It "$_" {
					echo "$signal -> $id" | should be $_
				}
			}
			elseif( $op -eq "LINK" )
			{
				It "$_" {
					echo "$source -> $id" | should be $_
				}
			}
			elseif( $op -eq "NOT" )
			{
				It "$_" {
					echo "NOT $source -> $id" | should be $_
				}
			}
			elseif( $op -eq "AND" )
			{
				It "$_" {
					echo "$source AND $source2 -> $id" | should be $_
				}
			}
			elseif( $op -eq "OR" )
			{
				It "$_" {
					echo "$source OR $source2 -> $id" | should be $_
				}
			}
			elseif( $op -eq "LSHIFT" )
			{
				It "$_" {
					echo "$source LSHIFT $source2 -> $id" | should be $_
				}
				It "source is numeric" {
					$source2 -match "^\d+$" | should be $true
				}
			}
			elseif( $op -eq "RSHIFT" )
			{
				It "$_" {
					echo "$source RSHIFT $source2 -> $id" | should be $_
				}
				It "source is numeric" {
					$source2 -match "^\d+$" | should be $true
				}
			}
		}
	}

	Describe "operators" {
		It "does and" {
			123 -band 456 | should be 72
		}
		It "does or" {
			123 -bor 456 | should be 507
		}
		It "does lshift" {
			123 -shl 2 -band 0xffff | should be 492
		}
		It "does rshift" {
			456 -shr 2 | should be 114
		}
		It "does not" {
			-bnot [uint]123 -band 0xffff | should be 65412
		}
		It "does not 2" {
			-bnot [uint]456 -band 0xffff | should be 65079
		}
	}
}