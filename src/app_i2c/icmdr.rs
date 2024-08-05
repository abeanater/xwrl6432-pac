#[doc = "Register `ICMDR` reader"]
pub type R = crate::R<IcmdrSpec>;
#[doc = "Register `ICMDR` writer"]
pub type W = crate::W<IcmdrSpec>;
#[doc = "Field `BC2_BC1_BC0` reader - 2:0\\]
Bit Count : Bit Count 2, Bit Count 1 and Bit Count 0 define the number of bits starting from the lsb (excluding the acknowledge bit) of the next byte which are yet to be received or transmitted. __________________________________________ BC2_BC1_BC0__Bits/byte in FDF__Bits/byte w/ ACK _0___0___1_____NA (reserved)____ NA (reserved) _0___1___0________2______________3_______ _0___1___1________3______________4_______ _1___0___0________4______________5_______ _1___0___1________5______________6_______ _1___1___0________6______________7_______ _1___1___1________7______________8_______ _0___0___0________8______________9_______ __________________________________________"]
pub type Bc2Bc1Bc0R = crate::FieldReader;
#[doc = "Field `BC2_BC1_BC0` writer - 2:0\\]
Bit Count : Bit Count 2, Bit Count 1 and Bit Count 0 define the number of bits starting from the lsb (excluding the acknowledge bit) of the next byte which are yet to be received or transmitted. __________________________________________ BC2_BC1_BC0__Bits/byte in FDF__Bits/byte w/ ACK _0___0___1_____NA (reserved)____ NA (reserved) _0___1___0________2______________3_______ _0___1___1________3______________4_______ _1___0___0________4______________5_______ _1___0___1________5______________6_______ _1___1___0________6______________7_______ _1___1___1________7______________8_______ _0___0___0________8______________9_______ __________________________________________"]
pub type Bc2Bc1Bc0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FDF` reader - 3:3\\]
Free Data Format. This bit can be set to\"1\" by the CPU to configure the I2C in Free Data Format mode. ______________________________________________ FDF___MST___TRX______Operating mode _0______0_____ x____Slave in non FDF mode _0______1_____0____Master receive in non FDF mode _0______1_____1____Master transmit in non FDF mode _1______0_____0____Slave receiver in FDF mode _1______0_____1____Slave transmitter in FDF mode _1______1_____0____Master receiver in FDF mode _1______1_____1____Master transmitter in FDF mode ______________________________________________"]
pub type FdfR = crate::BitReader;
#[doc = "Field `FDF` writer - 3:3\\]
Free Data Format. This bit can be set to\"1\" by the CPU to configure the I2C in Free Data Format mode. ______________________________________________ FDF___MST___TRX______Operating mode _0______0_____ x____Slave in non FDF mode _0______1_____0____Master receive in non FDF mode _0______1_____1____Master transmit in non FDF mode _1______0_____0____Slave receiver in FDF mode _1______0_____1____Slave transmitter in FDF mode _1______1_____0____Master receiver in FDF mode _1______1_____1____Master transmitter in FDF mode ______________________________________________"]
pub type FdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STB` reader - 4:4\\]
Start Byte (Master only mode). The Start Byte mode bit is set to 1 by the CPU to configure the I2C in Start byte mode the I2C sends \"00000001\"∩┐╜ regardless ICSAR value. Refer to the Philip I2C spec for more details."]
pub type StbR = crate::BitReader;
#[doc = "Field `STB` writer - 4:4\\]
Start Byte (Master only mode). The Start Byte mode bit is set to 1 by the CPU to configure the I2C in Start byte mode the I2C sends \"00000001\"∩┐╜ regardless ICSAR value. Refer to the Philip I2C spec for more details."]
pub type StbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRS` reader - 5:5\\]
I2C Reset Not. This can be set to a\"0\" by the CPU to put the I2C in reset or to a\"1\" to take the I2C out of reset. When this bit is reset to 0 all status bits in ICSTR and ICIVR are set to default values. Note that if this bit is reset during a transfer it can cause the I2C bus hang (SDA and SCL are tri-stated)."]
pub type IrsR = crate::BitReader;
#[doc = "Field `IRS` writer - 5:5\\]
I2C Reset Not. This can be set to a\"0\" by the CPU to put the I2C in reset or to a\"1\" to take the I2C out of reset. When this bit is reset to 0 all status bits in ICSTR and ICIVR are set to default values. Note that if this bit is reset during a transfer it can cause the I2C bus hang (SDA and SCL are tri-stated)."]
pub type IrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLB` reader - 6:6\\]
Digital Loop Back (in master transmit mode only). This bit is set to a\"1\" by the CPU to put the I2C in the loop back mode. In this mode data transmitted out of the ICDXR will be received in the ICDRR after ((CPU freq/I2C freq)8) CPU cycles via an internal path. The address of the ICOAR is output on SDA."]
pub type DlbR = crate::BitReader;
#[doc = "Field `DLB` writer - 6:6\\]
Digital Loop Back (in master transmit mode only). This bit is set to a\"1\" by the CPU to put the I2C in the loop back mode. In this mode data transmitted out of the ICDXR will be received in the ICDRR after ((CPU freq/I2C freq)8) CPU cycles via an internal path. The address of the ICOAR is output on SDA."]
pub type DlbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RM` reader - 7:7\\]
Repeat Mode. This bit is set to a\"1\" by the CPU to put the I2C in the repeat mode. In this mode data is continuously transmitted out of the ICDXR until the STP bit is set to\"1\" regardless of ICCNT value. This bit is don\"t care if the I2C is configured in slave mode. _________________________________________________________ RM___STT___STP___Conditions_____Bus Activities_______Mode _0_____0_____0_______Idle___________None___________NA _0_____0_____1_______Stop____________P____________NA _0_____1_____0_____(Re)Start_______S-A-D..(n)..D____Repeat n _0_____1_____1_____(Re)Start-Stop___S-A-D..(n)..D-P__Repeat n _1_____0_____0_______Idle___________none___________NA _1_____0_____1_______Stop____________P ___________NA _1_____1_____0_____(Re)Start_______S-A-D-D-D.._____Continuous _1_____1_____1_____Reserved________None___________NA _________________________________________________________"]
pub type RmR = crate::BitReader;
#[doc = "Field `RM` writer - 7:7\\]
Repeat Mode. This bit is set to a\"1\" by the CPU to put the I2C in the repeat mode. In this mode data is continuously transmitted out of the ICDXR until the STP bit is set to\"1\" regardless of ICCNT value. This bit is don\"t care if the I2C is configured in slave mode. _________________________________________________________ RM___STT___STP___Conditions_____Bus Activities_______Mode _0_____0_____0_______Idle___________None___________NA _0_____0_____1_______Stop____________P____________NA _0_____1_____0_____(Re)Start_______S-A-D..(n)..D____Repeat n _0_____1_____1_____(Re)Start-Stop___S-A-D..(n)..D-P__Repeat n _1_____0_____0_______Idle___________none___________NA _1_____0_____1_______Stop____________P ___________NA _1_____1_____0_____(Re)Start_______S-A-D-D-D.._____Continuous _1_____1_____1_____Reserved________None___________NA _________________________________________________________"]
pub type RmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XA` reader - 8:8\\]
Expanded Address. XA=0: (default) 7-bit address mode (normal address mode). XA=1: 10-bit address mode (expanded address mode) Please note that XA needs to be configured even if the I2C is in slave mode."]
pub type XaR = crate::BitReader;
#[doc = "Field `XA` writer - 8:8\\]
Expanded Address. XA=0: (default) 7-bit address mode (normal address mode). XA=1: 10-bit address mode (expanded address mode) Please note that XA needs to be configured even if the I2C is in slave mode."]
pub type XaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRX` reader - 9:9\\]
Transmitter. TRX=0: The I 2 C is in the\"receiver\" mode and data on data line SDA is shifted into the data register ICDRR. TRX=1: The I 2 C is in the\"transmitter\" mode and the data in ICDXR is shifted out on data line SDA. The operating modes (not in FDF mode) are defined as follows. In FDF mode TRX must be configured even if the I2C is in slave mode because there is no address/direction byte in FDF mode. ______________________________ MST___TRX___Operating Modes _0______x_____\"slave receiver\" _0______x_____\"slave transmitter\" _1______0_____\"master receiver\" _1______1_____\"master transmitter\" ______________________________"]
pub type TrxR = crate::BitReader;
#[doc = "Field `TRX` writer - 9:9\\]
Transmitter. TRX=0: The I 2 C is in the\"receiver\" mode and data on data line SDA is shifted into the data register ICDRR. TRX=1: The I 2 C is in the\"transmitter\" mode and the data in ICDXR is shifted out on data line SDA. The operating modes (not in FDF mode) are defined as follows. In FDF mode TRX must be configured even if the I2C is in slave mode because there is no address/direction byte in FDF mode. ______________________________ MST___TRX___Operating Modes _0______x_____\"slave receiver\" _0______x_____\"slave transmitter\" _1______0_____\"master receiver\" _1______1_____\"master transmitter\" ______________________________"]
pub type TrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST` reader - 10:10\\]
Master. MST=0: The I 2 C peripheral is in the\"slave\" mode and clock is received from the\"master\" device. MST=1: The I 2 C peripheral is in the\"master\" mode and it generates the clock. This bit is clear when the transfer completed."]
pub type MstR = crate::BitReader;
#[doc = "Field `MST` writer - 10:10\\]
Master. MST=0: The I 2 C peripheral is in the\"slave\" mode and clock is received from the\"master\" device. MST=1: The I 2 C peripheral is in the\"master\" mode and it generates the clock. This bit is clear when the transfer completed."]
pub type MstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP` reader - 11:11\\]
Stop Condition (Master mode only). This bit can be set to a\"1\" by the CPU to generate a Stop condition. It is reset to \"0\" by the hardware after the Stop condition has been generated. The Stop condition is generated when ICCNT passes 0 when the I2C is in non-repeat mode(RM=0)."]
pub type StpR = crate::BitReader;
#[doc = "Field `STP` writer - 11:11\\]
Stop Condition (Master mode only). This bit can be set to a\"1\" by the CPU to generate a Stop condition. It is reset to \"0\" by the hardware after the Stop condition has been generated. The Stop condition is generated when ICCNT passes 0 when the I2C is in non-repeat mode(RM=0)."]
pub type StpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 12:12\\]
Reserved for IDLEEN (IDLE Enable on 5509). - (RW )"]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - 12:12\\]
Reserved for IDLEEN (IDLE Enable on 5509). - (RW )"]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STT` reader - 13:13\\]
Start Condition (Master only mode). This bit can be set to a\"1\" by the CPU to generate a Start condition. In master mode when setting Start to\"1\" generates a Start condition. It is reset to \"0\" by the hardware after the Start condition has been generated. The Start/Stop bits can be configured to generate different transfer formats. Note that the STT and STP can be used to terminate the repeat mode. ____________________________________________________ STT___STP____Conditions_______________Bus Activities _1_____0________Start___________________S-A-D _0_____1________Stop_____________________P _1_____1________Start-Stop (ICCNT= n)______S-A-D..(n)..D-P _1_____0________Start (ICCNT= n)__________S-A-D..(n)..D ____________________________________________________"]
pub type SttR = crate::BitReader;
#[doc = "Field `STT` writer - 13:13\\]
Start Condition (Master only mode). This bit can be set to a\"1\" by the CPU to generate a Start condition. In master mode when setting Start to\"1\" generates a Start condition. It is reset to \"0\" by the hardware after the Start condition has been generated. The Start/Stop bits can be configured to generate different transfer formats. Note that the STT and STP can be used to terminate the repeat mode. ____________________________________________________ STT___STP____Conditions_______________Bus Activities _1_____0________Start___________________S-A-D _0_____1________Stop_____________________P _1_____1________Start-Stop (ICCNT= n)______S-A-D..(n)..D-P _1_____0________Start (ICCNT= n)__________S-A-D..(n)..D ____________________________________________________"]
pub type SttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREE` reader - 14:14\\]
Free Running. This bit is used to determine the state of the I2C when a breakpoint is encountered in the HLL debugger. FREE=0: (default) Stops immediately if SCL is low and keep driving SCL low whether I2C is master transmitter/receiver. If SCL is high I2C waits until SCL becomes low and then stops. If the I2C is a slave it will stop when the transmission/receiving completes. FREE=1: The I2C runs free."]
pub type FreeR = crate::BitReader;
#[doc = "Field `FREE` writer - 14:14\\]
Free Running. This bit is used to determine the state of the I2C when a breakpoint is encountered in the HLL debugger. FREE=0: (default) Stops immediately if SCL is low and keep driving SCL low whether I2C is master transmitter/receiver. If SCL is high I2C waits until SCL becomes low and then stops. If the I2C is a slave it will stop when the transmission/receiving completes. FREE=1: The I2C runs free."]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKMOD` reader - 15:15\\]
No Acknowledge (NACK) mode. This bit is used to send an Acknowledge (ACK) or a No Acknowledge (NACK) to the transmitter. This bit is only applicable when the I2C is in receiver mode. In master receiver mode when the internal data count counter decrements to zero the I2C sends a NACK. The master receiver I2C finishes a transfer when it sends a NACK. The I2C ignores ICCNT when NACKMOD is '1'. The NACKMOD bit should be set before the rising edge of the last data bit (bit 8) if a NACK must be sent and this bit is cleared once a NACK has been sent. NACKMOD=0 the I2C sends an ACK to the transmitter during the acknowledge cycle. NACKMOD=1 the I2C sends a NACK to the transmitter during the acknowledge cycle."]
pub type NackmodR = crate::BitReader;
#[doc = "Field `NACKMOD` writer - 15:15\\]
No Acknowledge (NACK) mode. This bit is used to send an Acknowledge (ACK) or a No Acknowledge (NACK) to the transmitter. This bit is only applicable when the I2C is in receiver mode. In master receiver mode when the internal data count counter decrements to zero the I2C sends a NACK. The master receiver I2C finishes a transfer when it sends a NACK. The I2C ignores ICCNT when NACKMOD is '1'. The NACKMOD bit should be set before the rising edge of the last data bit (bit 8) if a NACK must be sent and this bit is cleared once a NACK has been sent. NACKMOD=0 the I2C sends an ACK to the transmitter during the acknowledge cycle. NACKMOD=1 the I2C sends a NACK to the transmitter during the acknowledge cycle."]
pub type NackmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 31:16\\]
Reserved"]
pub type Nu2R = crate::FieldReader<u16>;
#[doc = "Field `NU2` writer - 31:16\\]
Reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Bit Count : Bit Count 2, Bit Count 1 and Bit Count 0 define the number of bits starting from the lsb (excluding the acknowledge bit) of the next byte which are yet to be received or transmitted. __________________________________________ BC2_BC1_BC0__Bits/byte in FDF__Bits/byte w/ ACK _0___0___1_____NA (reserved)____ NA (reserved) _0___1___0________2______________3_______ _0___1___1________3______________4_______ _1___0___0________4______________5_______ _1___0___1________5______________6_______ _1___1___0________6______________7_______ _1___1___1________7______________8_______ _0___0___0________8______________9_______ __________________________________________"]
    #[inline(always)]
    pub fn bc2_bc1_bc0(&self) -> Bc2Bc1Bc0R {
        Bc2Bc1Bc0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Free Data Format. This bit can be set to\"1\" by the CPU to configure the I2C in Free Data Format mode. ______________________________________________ FDF___MST___TRX______Operating mode _0______0_____ x____Slave in non FDF mode _0______1_____0____Master receive in non FDF mode _0______1_____1____Master transmit in non FDF mode _1______0_____0____Slave receiver in FDF mode _1______0_____1____Slave transmitter in FDF mode _1______1_____0____Master receiver in FDF mode _1______1_____1____Master transmitter in FDF mode ______________________________________________"]
    #[inline(always)]
    pub fn fdf(&self) -> FdfR {
        FdfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Start Byte (Master only mode). The Start Byte mode bit is set to 1 by the CPU to configure the I2C in Start byte mode the I2C sends \"00000001\"∩┐╜ regardless ICSAR value. Refer to the Philip I2C spec for more details."]
    #[inline(always)]
    pub fn stb(&self) -> StbR {
        StbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C Reset Not. This can be set to a\"0\" by the CPU to put the I2C in reset or to a\"1\" to take the I2C out of reset. When this bit is reset to 0 all status bits in ICSTR and ICIVR are set to default values. Note that if this bit is reset during a transfer it can cause the I2C bus hang (SDA and SCL are tri-stated)."]
    #[inline(always)]
    pub fn irs(&self) -> IrsR {
        IrsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Digital Loop Back (in master transmit mode only). This bit is set to a\"1\" by the CPU to put the I2C in the loop back mode. In this mode data transmitted out of the ICDXR will be received in the ICDRR after ((CPU freq/I2C freq)8) CPU cycles via an internal path. The address of the ICOAR is output on SDA."]
    #[inline(always)]
    pub fn dlb(&self) -> DlbR {
        DlbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Repeat Mode. This bit is set to a\"1\" by the CPU to put the I2C in the repeat mode. In this mode data is continuously transmitted out of the ICDXR until the STP bit is set to\"1\" regardless of ICCNT value. This bit is don\"t care if the I2C is configured in slave mode. _________________________________________________________ RM___STT___STP___Conditions_____Bus Activities_______Mode _0_____0_____0_______Idle___________None___________NA _0_____0_____1_______Stop____________P____________NA _0_____1_____0_____(Re)Start_______S-A-D..(n)..D____Repeat n _0_____1_____1_____(Re)Start-Stop___S-A-D..(n)..D-P__Repeat n _1_____0_____0_______Idle___________none___________NA _1_____0_____1_______Stop____________P ___________NA _1_____1_____0_____(Re)Start_______S-A-D-D-D.._____Continuous _1_____1_____1_____Reserved________None___________NA _________________________________________________________"]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Expanded Address. XA=0: (default) 7-bit address mode (normal address mode). XA=1: 10-bit address mode (expanded address mode) Please note that XA needs to be configured even if the I2C is in slave mode."]
    #[inline(always)]
    pub fn xa(&self) -> XaR {
        XaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter. TRX=0: The I 2 C is in the\"receiver\" mode and data on data line SDA is shifted into the data register ICDRR. TRX=1: The I 2 C is in the\"transmitter\" mode and the data in ICDXR is shifted out on data line SDA. The operating modes (not in FDF mode) are defined as follows. In FDF mode TRX must be configured even if the I2C is in slave mode because there is no address/direction byte in FDF mode. ______________________________ MST___TRX___Operating Modes _0______x_____\"slave receiver\" _0______x_____\"slave transmitter\" _1______0_____\"master receiver\" _1______1_____\"master transmitter\" ______________________________"]
    #[inline(always)]
    pub fn trx(&self) -> TrxR {
        TrxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Master. MST=0: The I 2 C peripheral is in the\"slave\" mode and clock is received from the\"master\" device. MST=1: The I 2 C peripheral is in the\"master\" mode and it generates the clock. This bit is clear when the transfer completed."]
    #[inline(always)]
    pub fn mst(&self) -> MstR {
        MstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Stop Condition (Master mode only). This bit can be set to a\"1\" by the CPU to generate a Stop condition. It is reset to \"0\" by the hardware after the Stop condition has been generated. The Stop condition is generated when ICCNT passes 0 when the I2C is in non-repeat mode(RM=0)."]
    #[inline(always)]
    pub fn stp(&self) -> StpR {
        StpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Reserved for IDLEEN (IDLE Enable on 5509). - (RW )"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Start Condition (Master only mode). This bit can be set to a\"1\" by the CPU to generate a Start condition. In master mode when setting Start to\"1\" generates a Start condition. It is reset to \"0\" by the hardware after the Start condition has been generated. The Start/Stop bits can be configured to generate different transfer formats. Note that the STT and STP can be used to terminate the repeat mode. ____________________________________________________ STT___STP____Conditions_______________Bus Activities _1_____0________Start___________________S-A-D _0_____1________Stop_____________________P _1_____1________Start-Stop (ICCNT= n)______S-A-D..(n)..D-P _1_____0________Start (ICCNT= n)__________S-A-D..(n)..D ____________________________________________________"]
    #[inline(always)]
    pub fn stt(&self) -> SttR {
        SttR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Free Running. This bit is used to determine the state of the I2C when a breakpoint is encountered in the HLL debugger. FREE=0: (default) Stops immediately if SCL is low and keep driving SCL low whether I2C is master transmitter/receiver. If SCL is high I2C waits until SCL becomes low and then stops. If the I2C is a slave it will stop when the transmission/receiving completes. FREE=1: The I2C runs free."]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
No Acknowledge (NACK) mode. This bit is used to send an Acknowledge (ACK) or a No Acknowledge (NACK) to the transmitter. This bit is only applicable when the I2C is in receiver mode. In master receiver mode when the internal data count counter decrements to zero the I2C sends a NACK. The master receiver I2C finishes a transfer when it sends a NACK. The I2C ignores ICCNT when NACKMOD is '1'. The NACKMOD bit should be set before the rising edge of the last data bit (bit 8) if a NACK must be sent and this bit is cleared once a NACK has been sent. NACKMOD=0 the I2C sends an ACK to the transmitter during the acknowledge cycle. NACKMOD=1 the I2C sends a NACK to the transmitter during the acknowledge cycle."]
    #[inline(always)]
    pub fn nackmod(&self) -> NackmodR {
        NackmodR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Bit Count : Bit Count 2, Bit Count 1 and Bit Count 0 define the number of bits starting from the lsb (excluding the acknowledge bit) of the next byte which are yet to be received or transmitted. __________________________________________ BC2_BC1_BC0__Bits/byte in FDF__Bits/byte w/ ACK _0___0___1_____NA (reserved)____ NA (reserved) _0___1___0________2______________3_______ _0___1___1________3______________4_______ _1___0___0________4______________5_______ _1___0___1________5______________6_______ _1___1___0________6______________7_______ _1___1___1________7______________8_______ _0___0___0________8______________9_______ __________________________________________"]
    #[inline(always)]
    #[must_use]
    pub fn bc2_bc1_bc0(&mut self) -> Bc2Bc1Bc0W<IcmdrSpec> {
        Bc2Bc1Bc0W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Free Data Format. This bit can be set to\"1\" by the CPU to configure the I2C in Free Data Format mode. ______________________________________________ FDF___MST___TRX______Operating mode _0______0_____ x____Slave in non FDF mode _0______1_____0____Master receive in non FDF mode _0______1_____1____Master transmit in non FDF mode _1______0_____0____Slave receiver in FDF mode _1______0_____1____Slave transmitter in FDF mode _1______1_____0____Master receiver in FDF mode _1______1_____1____Master transmitter in FDF mode ______________________________________________"]
    #[inline(always)]
    #[must_use]
    pub fn fdf(&mut self) -> FdfW<IcmdrSpec> {
        FdfW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Start Byte (Master only mode). The Start Byte mode bit is set to 1 by the CPU to configure the I2C in Start byte mode the I2C sends \"00000001\"∩┐╜ regardless ICSAR value. Refer to the Philip I2C spec for more details."]
    #[inline(always)]
    #[must_use]
    pub fn stb(&mut self) -> StbW<IcmdrSpec> {
        StbW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C Reset Not. This can be set to a\"0\" by the CPU to put the I2C in reset or to a\"1\" to take the I2C out of reset. When this bit is reset to 0 all status bits in ICSTR and ICIVR are set to default values. Note that if this bit is reset during a transfer it can cause the I2C bus hang (SDA and SCL are tri-stated)."]
    #[inline(always)]
    #[must_use]
    pub fn irs(&mut self) -> IrsW<IcmdrSpec> {
        IrsW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Digital Loop Back (in master transmit mode only). This bit is set to a\"1\" by the CPU to put the I2C in the loop back mode. In this mode data transmitted out of the ICDXR will be received in the ICDRR after ((CPU freq/I2C freq)8) CPU cycles via an internal path. The address of the ICOAR is output on SDA."]
    #[inline(always)]
    #[must_use]
    pub fn dlb(&mut self) -> DlbW<IcmdrSpec> {
        DlbW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Repeat Mode. This bit is set to a\"1\" by the CPU to put the I2C in the repeat mode. In this mode data is continuously transmitted out of the ICDXR until the STP bit is set to\"1\" regardless of ICCNT value. This bit is don\"t care if the I2C is configured in slave mode. _________________________________________________________ RM___STT___STP___Conditions_____Bus Activities_______Mode _0_____0_____0_______Idle___________None___________NA _0_____0_____1_______Stop____________P____________NA _0_____1_____0_____(Re)Start_______S-A-D..(n)..D____Repeat n _0_____1_____1_____(Re)Start-Stop___S-A-D..(n)..D-P__Repeat n _1_____0_____0_______Idle___________none___________NA _1_____0_____1_______Stop____________P ___________NA _1_____1_____0_____(Re)Start_______S-A-D-D-D.._____Continuous _1_____1_____1_____Reserved________None___________NA _________________________________________________________"]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RmW<IcmdrSpec> {
        RmW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Expanded Address. XA=0: (default) 7-bit address mode (normal address mode). XA=1: 10-bit address mode (expanded address mode) Please note that XA needs to be configured even if the I2C is in slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn xa(&mut self) -> XaW<IcmdrSpec> {
        XaW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter. TRX=0: The I 2 C is in the\"receiver\" mode and data on data line SDA is shifted into the data register ICDRR. TRX=1: The I 2 C is in the\"transmitter\" mode and the data in ICDXR is shifted out on data line SDA. The operating modes (not in FDF mode) are defined as follows. In FDF mode TRX must be configured even if the I2C is in slave mode because there is no address/direction byte in FDF mode. ______________________________ MST___TRX___Operating Modes _0______x_____\"slave receiver\" _0______x_____\"slave transmitter\" _1______0_____\"master receiver\" _1______1_____\"master transmitter\" ______________________________"]
    #[inline(always)]
    #[must_use]
    pub fn trx(&mut self) -> TrxW<IcmdrSpec> {
        TrxW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Master. MST=0: The I 2 C peripheral is in the\"slave\" mode and clock is received from the\"master\" device. MST=1: The I 2 C peripheral is in the\"master\" mode and it generates the clock. This bit is clear when the transfer completed."]
    #[inline(always)]
    #[must_use]
    pub fn mst(&mut self) -> MstW<IcmdrSpec> {
        MstW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Stop Condition (Master mode only). This bit can be set to a\"1\" by the CPU to generate a Stop condition. It is reset to \"0\" by the hardware after the Stop condition has been generated. The Stop condition is generated when ICCNT passes 0 when the I2C is in non-repeat mode(RM=0)."]
    #[inline(always)]
    #[must_use]
    pub fn stp(&mut self) -> StpW<IcmdrSpec> {
        StpW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Reserved for IDLEEN (IDLE Enable on 5509). - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IcmdrSpec> {
        Nu1W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Start Condition (Master only mode). This bit can be set to a\"1\" by the CPU to generate a Start condition. In master mode when setting Start to\"1\" generates a Start condition. It is reset to \"0\" by the hardware after the Start condition has been generated. The Start/Stop bits can be configured to generate different transfer formats. Note that the STT and STP can be used to terminate the repeat mode. ____________________________________________________ STT___STP____Conditions_______________Bus Activities _1_____0________Start___________________S-A-D _0_____1________Stop_____________________P _1_____1________Start-Stop (ICCNT= n)______S-A-D..(n)..D-P _1_____0________Start (ICCNT= n)__________S-A-D..(n)..D ____________________________________________________"]
    #[inline(always)]
    #[must_use]
    pub fn stt(&mut self) -> SttW<IcmdrSpec> {
        SttW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Free Running. This bit is used to determine the state of the I2C when a breakpoint is encountered in the HLL debugger. FREE=0: (default) Stops immediately if SCL is low and keep driving SCL low whether I2C is master transmitter/receiver. If SCL is high I2C waits until SCL becomes low and then stops. If the I2C is a slave it will stop when the transmission/receiving completes. FREE=1: The I2C runs free."]
    #[inline(always)]
    #[must_use]
    pub fn free(&mut self) -> FreeW<IcmdrSpec> {
        FreeW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
No Acknowledge (NACK) mode. This bit is used to send an Acknowledge (ACK) or a No Acknowledge (NACK) to the transmitter. This bit is only applicable when the I2C is in receiver mode. In master receiver mode when the internal data count counter decrements to zero the I2C sends a NACK. The master receiver I2C finishes a transfer when it sends a NACK. The I2C ignores ICCNT when NACKMOD is '1'. The NACKMOD bit should be set before the rising edge of the last data bit (bit 8) if a NACK must be sent and this bit is cleared once a NACK has been sent. NACKMOD=0 the I2C sends an ACK to the transmitter during the acknowledge cycle. NACKMOD=1 the I2C sends a NACK to the transmitter during the acknowledge cycle."]
    #[inline(always)]
    #[must_use]
    pub fn nackmod(&mut self) -> NackmodW<IcmdrSpec> {
        NackmodW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<IcmdrSpec> {
        Nu2W::new(self, 16)
    }
}
#[doc = "I2C Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`icmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmdrSpec;
impl crate::RegisterSpec for IcmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icmdr::R`](R) reader structure"]
impl crate::Readable for IcmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`icmdr::W`](W) writer structure"]
impl crate::Writable for IcmdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICMDR to value 0"]
impl crate::Resettable for IcmdrSpec {
    const RESET_VALUE: u32 = 0;
}
