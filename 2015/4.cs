using System;
using System.Text;

namespace md5
{
	class Program
	{
		public static string ToStr(byte[] hash)
		{
			StringBuilder sb = new StringBuilder();
			for (int i = 0; i < hash.Length; i++)
			{
				sb.Append(hash[i].ToString("X2"));
			}
			return sb.ToString();
		}

		public static byte[] CalculateMD5Hash(string input)
		{
			using (System.Security.Cryptography.MD5 md5 = System.Security.Cryptography.MD5.Create())
			{
				byte[] inputBytes = System.Text.Encoding.ASCII.GetBytes(input);
				byte[] hashBytes = md5.ComputeHash(inputBytes);
				return hashBytes;
			}
		}


		static void Main(string[] args)
		{
			string input = "iwrupvqb";
			int i = 1;
			while (true)
			{
				var result = CalculateMD5Hash(input + i);
				if (result[0] == 0 && result[1] == 0 && ((result[2] & 0xf0) == 0))
				{
					Console.WriteLine("{0}{1} becomes {2}", input, i, ToStr(result));
					break;
				}
				if (i % 1000 == 0)
				{
					Console.Write(".");
				}
				i++;
			}
			while (true)
			{
				var result = CalculateMD5Hash(input + i);
				if (result[0] == 0 && result[1] == 0 && (result[2] == 0))
				{
					Console.WriteLine("{0}{1} becomes {2}", input, i, ToStr(result));
					break;
				}
				if (i % 1000 == 0)
				{
					Console.Write(".");
				}
				i++;
			}
		}
	}
}