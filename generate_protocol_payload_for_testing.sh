echo "----------- 1. Preparaing protocol request packet ---------------- "

# listen for udp packets in 127.0.0.1:4321 in the background
nc -u -l -p 4321 >dns_req_packet &

# send 1 dns request i.e udp packet to the 127.0.0.1:4321
sleep 1 && dig @127.0.0.1 -p 4321 +retry=0 +noedns github.com

# kill the background netcat process
sleep 5 &&
	kill -KILL $(ps | grep nc | awk 'NR==1{print $1}')

echo "----------- 2. Preparaing protocol response packet ---------------- "

# send the dns (udp) request to the known dns server and capture the response packet
nc -u 1.1.1.1 53 <dns_req_packet >dns_res_packet &

# kill the background netcat process
sleep 7 &&
	kill -KILL $(ps | grep nc | awk 'NR==1{print $1}')

echo "-- \nGeneration Success!!"

echo "-------------- dns_req_packet ---------------------"
hexdump -C dns_req_packet

echo "-------------- dns_res_packet ---------------------"
hexdump -C dns_res_packet
