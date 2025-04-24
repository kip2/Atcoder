program main
   implicit none
   integer :: a, b, c

   read(*,*) a, b, c

   if (a*a + b*b < c*c) then
      print '(A)', 'Yes'
   else
      print '(A)', 'No'
   end if
end program main

