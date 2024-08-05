#[doc = "Register `ICEMDR` reader"]
pub type R = crate::R<IcemdrSpec>;
#[doc = "Register `ICEMDR` writer"]
pub type W = crate::W<IcemdrSpec>;
#[doc = "Field `BCM` reader - 0:0\\]
Backward Compatibility Mode. This bit affects the I2C interrupt behavior. Refer to appendix A for details."]
pub type BcmR = crate::BitReader;
#[doc = "Field `BCM` writer - 0:0\\]
Backward Compatibility Mode. This bit affects the I2C interrupt behavior. Refer to appendix A for details."]
pub type BcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNACK` reader - 1:1\\]
Ignore NACK mode IGNACK=0 The master transmitter will operate normally discontinue the data transfer and set the ARDY and NACK status bits when a NACK signal is received from the slave. IGNACK=1 The master transmitter will ignore a NACK received from the slave."]
pub type IgnackR = crate::BitReader;
#[doc = "Field `IGNACK` writer - 1:1\\]
Ignore NACK mode IGNACK=0 The master transmitter will operate normally discontinue the data transfer and set the ARDY and NACK status bits when a NACK signal is received from the slave. IGNACK=1 The master transmitter will ignore a NACK received from the slave."]
pub type IgnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:2\\]
Reserved. - (RW )"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:2\\]
Reserved. - (RW )"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Backward Compatibility Mode. This bit affects the I2C interrupt behavior. Refer to appendix A for details."]
    #[inline(always)]
    pub fn bcm(&self) -> BcmR {
        BcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Ignore NACK mode IGNACK=0 The master transmitter will operate normally discontinue the data transfer and set the ARDY and NACK status bits when a NACK signal is received from the slave. IGNACK=1 The master transmitter will ignore a NACK received from the slave."]
    #[inline(always)]
    pub fn ignack(&self) -> IgnackR {
        IgnackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved. - (RW )"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Backward Compatibility Mode. This bit affects the I2C interrupt behavior. Refer to appendix A for details."]
    #[inline(always)]
    #[must_use]
    pub fn bcm(&mut self) -> BcmW<IcemdrSpec> {
        BcmW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Ignore NACK mode IGNACK=0 The master transmitter will operate normally discontinue the data transfer and set the ARDY and NACK status bits when a NACK signal is received from the slave. IGNACK=1 The master transmitter will ignore a NACK received from the slave."]
    #[inline(always)]
    #[must_use]
    pub fn ignack(&mut self) -> IgnackW<IcemdrSpec> {
        IgnackW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved. - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcemdrSpec> {
        NuW::new(self, 2)
    }
}
#[doc = "I2C Extended Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`icemdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icemdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcemdrSpec;
impl crate::RegisterSpec for IcemdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icemdr::R`](R) reader structure"]
impl crate::Readable for IcemdrSpec {}
#[doc = "`write(|w| ..)` method takes [`icemdr::W`](W) writer structure"]
impl crate::Writable for IcemdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICEMDR to value 0"]
impl crate::Resettable for IcemdrSpec {
    const RESET_VALUE: u32 = 0;
}
