import mimc.(Mimc);

circuit Cipher{
   function decrypt(element: field, key: field, IV: field) -> field{
      return element - Mimc::mimc7(key,IV);
   }
}

