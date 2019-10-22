$data = gc 8.txt

function TrimEsc
{
	param($str)

	return $str.substring(1,$str.length-2) -replace "\\\\","\" -replace "\\""","""" -replace "\\x[0-9a-f]{2}","."
}

function Expnd
{
	param($str)

	$expanded = $str -replace "\\","\\" -replace """","\"""
	return """$expanded"""
}
Describe "expanding" {
	It "handles \""" {
		Expnd """aaa\""aaa""" | should be """\""aaa\\\""aaa\"""""
	}
}
Describe "escaping" {
	It "handles \" {
		TrimEsc ".\\." | should be "\"
	}
	It "handles """ {
		TrimEsc ".\""." | should be """"
	}
	It "cheats on \x" {
		TrimEsc ".\x33." | should be "."
	}
}

$data | %{
	$fullcount = $_.length
	$realcount = (TrimEsc $_).length
	return $fullcount - $realcount
} | measure -sum

$data | %{
	$fullcount = $_.length
	$realcount = (Expnd $_).length
	return $realcount - $fullcount
} | measure -sum