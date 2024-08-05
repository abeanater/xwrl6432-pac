#[doc = "Register `ICIVR` reader"]
pub type R = crate::R<IcivrSpec>;
#[doc = "Register `ICIVR` writer"]
pub type W = crate::W<IcivrSpec>;
#[doc = "Field `INTCODE` reader - 2:0\\]
Interrupt code. The binary-coded-interrupt vector indicates which interrupt has occurred. Reading the ICIVR clears the interrupt code except ARDY(011) RRDY(100) and XRDY(101). Interrupt code for ARDY RRDY and XRDY is cleared when ARDY ICRRDY and ICXRDY bits in the ICSTR is cleared to default value respectively. If other interrupts are pending a new interrupt is generated. If there are more than one interrupt flag reading the ICIVR clears the highest priority interrupt code. Reading the ICIVR also clears corresponding status bit in the ICSTR except ARDY ICRRDY ICXRDY and AAS. Note that users must read (clear) the ICIVR before doing another start otherwise the ICIVR could contain incorrect (old interrupt flags) value. ________________________________________________ Interrupt Code____________Interrupt Occurred__________ _000_(default)_________________None _001_(highest priority)____Arbitration Lost interrupt _010__________________No Acknowledgement interrupt _011__________________Register Access Ready interrupt _100__________________Receive Data Ready interrupt _101__________________Transmit Data Ready interrupt _110__________________Stop Condition Detection _111_(lowest priority)_____Address As Slave - (RW) ________________________________________________"]
pub type IntcodeR = crate::FieldReader;
#[doc = "Field `INTCODE` writer - 2:0\\]
Interrupt code. The binary-coded-interrupt vector indicates which interrupt has occurred. Reading the ICIVR clears the interrupt code except ARDY(011) RRDY(100) and XRDY(101). Interrupt code for ARDY RRDY and XRDY is cleared when ARDY ICRRDY and ICXRDY bits in the ICSTR is cleared to default value respectively. If other interrupts are pending a new interrupt is generated. If there are more than one interrupt flag reading the ICIVR clears the highest priority interrupt code. Reading the ICIVR also clears corresponding status bit in the ICSTR except ARDY ICRRDY ICXRDY and AAS. Note that users must read (clear) the ICIVR before doing another start otherwise the ICIVR could contain incorrect (old interrupt flags) value. ________________________________________________ Interrupt Code____________Interrupt Occurred__________ _000_(default)_________________None _001_(highest priority)____Arbitration Lost interrupt _010__________________No Acknowledgement interrupt _011__________________Register Access Ready interrupt _100__________________Receive Data Ready interrupt _101__________________Transmit Data Ready interrupt _110__________________Stop Condition Detection _111_(lowest priority)_____Address As Slave - (RW) ________________________________________________"]
pub type IntcodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU1` reader - 7:3\\]
Reserved."]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:3\\]
Reserved."]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TESTMD` reader - 11:8\\]
Reserved for internal testing."]
pub type TestmdR = crate::FieldReader;
#[doc = "Field `TESTMD` writer - 11:8\\]
Reserved for internal testing."]
pub type TestmdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU2` reader - 31:12\\]
Reserved."]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - 31:12\\]
Reserved."]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Interrupt code. The binary-coded-interrupt vector indicates which interrupt has occurred. Reading the ICIVR clears the interrupt code except ARDY(011) RRDY(100) and XRDY(101). Interrupt code for ARDY RRDY and XRDY is cleared when ARDY ICRRDY and ICXRDY bits in the ICSTR is cleared to default value respectively. If other interrupts are pending a new interrupt is generated. If there are more than one interrupt flag reading the ICIVR clears the highest priority interrupt code. Reading the ICIVR also clears corresponding status bit in the ICSTR except ARDY ICRRDY ICXRDY and AAS. Note that users must read (clear) the ICIVR before doing another start otherwise the ICIVR could contain incorrect (old interrupt flags) value. ________________________________________________ Interrupt Code____________Interrupt Occurred__________ _000_(default)_________________None _001_(highest priority)____Arbitration Lost interrupt _010__________________No Acknowledgement interrupt _011__________________Register Access Ready interrupt _100__________________Receive Data Ready interrupt _101__________________Transmit Data Ready interrupt _110__________________Stop Condition Detection _111_(lowest priority)_____Address As Slave - (RW) ________________________________________________"]
    #[inline(always)]
    pub fn intcode(&self) -> IntcodeR {
        IntcodeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Reserved."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Reserved for internal testing."]
    #[inline(always)]
    pub fn testmd(&self) -> TestmdR {
        TestmdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Interrupt code. The binary-coded-interrupt vector indicates which interrupt has occurred. Reading the ICIVR clears the interrupt code except ARDY(011) RRDY(100) and XRDY(101). Interrupt code for ARDY RRDY and XRDY is cleared when ARDY ICRRDY and ICXRDY bits in the ICSTR is cleared to default value respectively. If other interrupts are pending a new interrupt is generated. If there are more than one interrupt flag reading the ICIVR clears the highest priority interrupt code. Reading the ICIVR also clears corresponding status bit in the ICSTR except ARDY ICRRDY ICXRDY and AAS. Note that users must read (clear) the ICIVR before doing another start otherwise the ICIVR could contain incorrect (old interrupt flags) value. ________________________________________________ Interrupt Code____________Interrupt Occurred__________ _000_(default)_________________None _001_(highest priority)____Arbitration Lost interrupt _010__________________No Acknowledgement interrupt _011__________________Register Access Ready interrupt _100__________________Receive Data Ready interrupt _101__________________Transmit Data Ready interrupt _110__________________Stop Condition Detection _111_(lowest priority)_____Address As Slave - (RW) ________________________________________________"]
    #[inline(always)]
    #[must_use]
    pub fn intcode(&mut self) -> IntcodeW<IcivrSpec> {
        IntcodeW::new(self, 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IcivrSpec> {
        Nu1W::new(self, 3)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Reserved for internal testing."]
    #[inline(always)]
    #[must_use]
    pub fn testmd(&mut self) -> TestmdW<IcivrSpec> {
        TestmdW::new(self, 8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<IcivrSpec> {
        Nu2W::new(self, 12)
    }
}
#[doc = "I2C Interrupt Vector register\n\nYou can [`read`](crate::Reg::read) this register and get [`icivr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icivr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcivrSpec;
impl crate::RegisterSpec for IcivrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icivr::R`](R) reader structure"]
impl crate::Readable for IcivrSpec {}
#[doc = "`write(|w| ..)` method takes [`icivr::W`](W) writer structure"]
impl crate::Writable for IcivrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICIVR to value 0"]
impl crate::Resettable for IcivrSpec {
    const RESET_VALUE: u32 = 0;
}
