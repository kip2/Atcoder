program main
   implicit none
   character(len=100) :: input, rotated
   character(len=1) :: head
   character(len=99) :: tail
   integer :: len_input

   read(*, '(A)') input
   len_input = len_trim(input)

   head = input(1:1)
   tail = input(2:len_input)

   rotated = trim(tail) // head
   print '(A)', trim(rotated)
end program main
