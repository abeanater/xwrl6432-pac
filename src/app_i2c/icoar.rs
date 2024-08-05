#[doc = "Register `ICOAR` reader"]
pub type R = crate::R<IcoarSpec>;
#[doc = "Register `ICOAR` writer"]
pub type W = crate::W<IcoarSpec>;
#[doc = "Field `A9_A0` reader - 9:0\\]
Own address. Use in both 7- and 10-bit address mode. Note that usercan program the I2C own address to any value as long as it does notconflict with other components in the system."]
pub type A9A0R = crate::FieldReader<u16>;
#[doc = "Field `A9_A0` writer - 9:0\\]
Own address. Use in both 7- and 10-bit address mode. Note that usercan program the I2C own address to any value as long as it does notconflict with other components in the system."]
pub type A9A0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NU` reader - 31:10\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:10\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Own address. Use in both 7- and 10-bit address mode. Note that usercan program the I2C own address to any value as long as it does notconflict with other components in the system."]
    #[inline(always)]
    pub fn a9_a0(&self) -> A9A0R {
        A9A0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Own address. Use in both 7- and 10-bit address mode. Note that usercan program the I2C own address to any value as long as it does notconflict with other components in the system."]
    #[inline(always)]
    #[must_use]
    pub fn a9_a0(&mut self) -> A9A0W<IcoarSpec> {
        A9A0W::new(self, 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcoarSpec> {
        NuW::new(self, 10)
    }
}
#[doc = "I2C Own Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`icoar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icoar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcoarSpec;
impl crate::RegisterSpec for IcoarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icoar::R`](R) reader structure"]
impl crate::Readable for IcoarSpec {}
#[doc = "`write(|w| ..)` method takes [`icoar::W`](W) writer structure"]
impl crate::Writable for IcoarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICOAR to value 0"]
impl crate::Resettable for IcoarSpec {
    const RESET_VALUE: u32 = 0;
}
