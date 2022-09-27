reset

#set terminal png size 500, 500
#set output 'profit_loss.png'

#set terminal svg size 500, 500
#set output 'profit_loss.svg'

#set terminal eps size 500, 500
#set output 'profit_loss.eps'

set terminal pdfcairo
set output 'profit_loss.pdf'

set title 'FTNT strategy P\&L for the 25 October 2022'
set xlabel 'Stock price, $'
set ylabel 'P\&L, $'
set xzeroaxis
set grid
unset key

plot "profit_loss.txt" with lines lt -1 lw 2


