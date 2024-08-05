#[doc = "Register `ICIMR` reader"]
pub type R = crate::R<IcimrSpec>;
#[doc = "Register `ICIMR` writer"]
pub type W = crate::W<IcimrSpec>;
#[doc = "Field `AL` reader - 0:0\\]
Arbitration Lost interrupt mask bit. Setting a\"1\" to this bit unmasks the Arbitration Lost interrupt. Setting a\"0\" to this bit masks the Arbitration Lost interrupt."]
pub type AlR = crate::BitReader;
#[doc = "Field `AL` writer - 0:0\\]
Arbitration Lost interrupt mask bit. Setting a\"1\" to this bit unmasks the Arbitration Lost interrupt. Setting a\"0\" to this bit masks the Arbitration Lost interrupt."]
pub type AlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - 1:1\\]
No Acknowledgement interrupt mask bit. Setting a\"1\" to this bit unmasks the No Acknowledgement interrupt. Setting a\"0\" to this bit masks the No Acknowledgement interrupt."]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - 1:1\\]
No Acknowledgement interrupt mask bit. Setting a\"1\" to this bit unmasks the No Acknowledgement interrupt. Setting a\"0\" to this bit masks the No Acknowledgement interrupt."]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDY` reader - 2:2\\]
Register access ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Register access ready interrupt. Setting a\"0\" to this bit masks the Register access ready interrupt."]
pub type ArdyR = crate::BitReader;
#[doc = "Field `ARDY` writer - 2:2\\]
Register access ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Register access ready interrupt. Setting a\"0\" to this bit masks the Register access ready interrupt."]
pub type ArdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICRRDY` reader - 3:3\\]
Receive Data Ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Receive Data Ready interrupt. Setting a\"0\" to this bit masks the Receive Data Ready interrupt."]
pub type IcrrdyR = crate::BitReader;
#[doc = "Field `ICRRDY` writer - 3:3\\]
Receive Data Ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Receive Data Ready interrupt. Setting a\"0\" to this bit masks the Receive Data Ready interrupt."]
pub type IcrrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICXRDY` reader - 4:4\\]
Transmit Data Ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Transmit Data Ready interrupt. Setting a\"0\" to this bit masks the Transmit Data Ready interrupt."]
pub type IcxrdyR = crate::BitReader;
#[doc = "Field `ICXRDY` writer - 4:4\\]
Transmit Data Ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Transmit Data Ready interrupt. Setting a\"0\" to this bit masks the Transmit Data Ready interrupt."]
pub type IcxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCD` reader - 5:5\\]
Stop Condition Detection mask bit. Setting a\"1\" to this bit unmasks the Stop Condition Detection interrupt. Setting a \"0\" to this bit masks the Stop Condition Detection interrupt."]
pub type ScdR = crate::BitReader;
#[doc = "Field `SCD` writer - 5:5\\]
Stop Condition Detection mask bit. Setting a\"1\" to this bit unmasks the Stop Condition Detection interrupt. Setting a \"0\" to this bit masks the Stop Condition Detection interrupt."]
pub type ScdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAS` reader - 6:6\\]
Address As Slave interrupt mask bit. Setting a\"1\" to this bit unmasks the Address As Slave interrupt. Setting a\"0\" to this bit masks the Address As Slave interrupt."]
pub type AasR = crate::BitReader;
#[doc = "Field `AAS` writer - 6:6\\]
Address As Slave interrupt mask bit. Setting a\"1\" to this bit unmasks the Address As Slave interrupt. Setting a\"0\" to this bit masks the Address As Slave interrupt."]
pub type AasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:7\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:7\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Arbitration Lost interrupt mask bit. Setting a\"1\" to this bit unmasks the Arbitration Lost interrupt. Setting a\"0\" to this bit masks the Arbitration Lost interrupt."]
    #[inline(always)]
    pub fn al(&self) -> AlR {
        AlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
No Acknowledgement interrupt mask bit. Setting a\"1\" to this bit unmasks the No Acknowledgement interrupt. Setting a\"0\" to this bit masks the No Acknowledgement interrupt."]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Register access ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Register access ready interrupt. Setting a\"0\" to this bit masks the Register access ready interrupt."]
    #[inline(always)]
    pub fn ardy(&self) -> ArdyR {
        ArdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive Data Ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Receive Data Ready interrupt. Setting a\"0\" to this bit masks the Receive Data Ready interrupt."]
    #[inline(always)]
    pub fn icrrdy(&self) -> IcrrdyR {
        IcrrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit Data Ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Transmit Data Ready interrupt. Setting a\"0\" to this bit masks the Transmit Data Ready interrupt."]
    #[inline(always)]
    pub fn icxrdy(&self) -> IcxrdyR {
        IcxrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Stop Condition Detection mask bit. Setting a\"1\" to this bit unmasks the Stop Condition Detection interrupt. Setting a \"0\" to this bit masks the Stop Condition Detection interrupt."]
    #[inline(always)]
    pub fn scd(&self) -> ScdR {
        ScdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Address As Slave interrupt mask bit. Setting a\"1\" to this bit unmasks the Address As Slave interrupt. Setting a\"0\" to this bit masks the Address As Slave interrupt."]
    #[inline(always)]
    pub fn aas(&self) -> AasR {
        AasR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Arbitration Lost interrupt mask bit. Setting a\"1\" to this bit unmasks the Arbitration Lost interrupt. Setting a\"0\" to this bit masks the Arbitration Lost interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn al(&mut self) -> AlW<IcimrSpec> {
        AlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
No Acknowledgement interrupt mask bit. Setting a\"1\" to this bit unmasks the No Acknowledgement interrupt. Setting a\"0\" to this bit masks the No Acknowledgement interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<IcimrSpec> {
        NackW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Register access ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Register access ready interrupt. Setting a\"0\" to this bit masks the Register access ready interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ardy(&mut self) -> ArdyW<IcimrSpec> {
        ArdyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive Data Ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Receive Data Ready interrupt. Setting a\"0\" to this bit masks the Receive Data Ready interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icrrdy(&mut self) -> IcrrdyW<IcimrSpec> {
        IcrrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit Data Ready interrupt mask bit. Setting a\"1\" to this bit unmasks the Transmit Data Ready interrupt. Setting a\"0\" to this bit masks the Transmit Data Ready interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icxrdy(&mut self) -> IcxrdyW<IcimrSpec> {
        IcxrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Stop Condition Detection mask bit. Setting a\"1\" to this bit unmasks the Stop Condition Detection interrupt. Setting a \"0\" to this bit masks the Stop Condition Detection interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn scd(&mut self) -> ScdW<IcimrSpec> {
        ScdW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Address As Slave interrupt mask bit. Setting a\"1\" to this bit unmasks the Address As Slave interrupt. Setting a\"0\" to this bit masks the Address As Slave interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn aas(&mut self) -> AasW<IcimrSpec> {
        AasW::new(self, 6)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcimrSpec> {
        NuW::new(self, 7)
    }
}
#[doc = "I2C Interrupt Mask/Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`icimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcimrSpec;
impl crate::RegisterSpec for IcimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icimr::R`](R) reader structure"]
impl crate::Readable for IcimrSpec {}
#[doc = "`write(|w| ..)` method takes [`icimr::W`](W) writer structure"]
impl crate::Writable for IcimrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICIMR to value 0"]
impl crate::Resettable for IcimrSpec {
    const RESET_VALUE: u32 = 0;
}
