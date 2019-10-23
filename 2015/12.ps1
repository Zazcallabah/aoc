$data = gc -raw 12.txt | convertfrom-json

function Handle
{
	param($obj,[ref]$numbers,[switch]$filter)

	$typename = $obj.GetType().Name

	if( $typename -eq "Object[]" )
	{
		$obj | %{ Handle $_ $numbers -filter:$filter }
	}
	elseif( $typename -eq "PSCustomObject" )
	{
		if( $obj.PsObject.Properties.Name -eq $null )
		{
			return
		}
		if( $filter )
		{
			foreach($prop in $obj.PsObject.Properties.Name )
			{
				if( $obj."$prop".Equals( "red" ) )
				{
					return
				}
			}
		}
		$obj.PsObject.Properties.Name | %{
			Handle $obj."$_" $numbers -filter:$filter
		}
	}
	elseif( $typename -ne "String" )
	{
		$numbers.value += $obj
	}
}

function Test
{
	param($str,$expected,[switch]$filter)
	$n = @()
	Handle ($str|convertfrom-json) ([ref]$n) -filter:$filter
	$n | measure -sum | select -expandproperty sum | should be $expected
}
Describe "filter" {
	It "handles simple arr" {
		Test @(1,2,3) 6 -filter
	}
	It "skips obj with red prop" {
		Test '[1,{"c":"red","b":2},3]' 4 -filter
	}
	It "skips entire structure if needed" {
		Test '{"d":"red","e":[1,2,3,4],"f":5}' 0 -filter
	}
	It "doesnt skip arr" {
		Test '[1,"red",5]' 6 -filter
	}
	It "really doesnt skip arr" {
		Test '{"a":[1,"red",5],"b":{"a":2,"b":[1,2,"red"]}}' 11 -filter
	}
}

Describe "nofilter" {
	It "handles simple arr" {
		Test @(1,2,3) 6
	}
	It "handles simple obj" {
		Test '{"a":2,"b":4}' 6
	}
	It "handles nested arr" {
		Test '[[[3]]]' 3
	}
	It "handles nested obj" {
		Test '{"a":{"b":4},"c":-1}' 3
	}
	It "handles empty obj" {
		Test "{}" 0
	}
	It "handles empty arr" {
		Test "[]" 0
	}
	It "handles negative numbers 1" {
		Test '{"a":[-1,1]}' 0
	}
	It "handles negative numbers 2" {
		Test '[-1,{"a":1}]' 0
	}
}

$numbers = @()
Handle $data ([ref]$numbers)
$numbers | measure -sum

$numbers = @()
Handle $data ([ref]$numbers) -filter
$numbers | measure -sum