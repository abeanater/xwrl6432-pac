#[doc = "Register `GIOPWDN` reader"]
pub type R = crate::R<GiopwdnSpec>;
#[doc = "Register `GIOPWDN` writer"]
pub type W = crate::W<GiopwdnSpec>;
#[doc = "Field `GIOPWDN` reader - 0:0\\]
Writing to the GIOPWDN bit is only allowed in privilege mode. Reading of the GIOPWDN bit is allowed in all modes. Privilege mode (write): 0 = Normal operation; clocks enabled to GIO module 1 = Power-down mode User mode (write): Writes have no effect in user mode. User or privilege mode (read): 0 = Normal operation; clocks enabled to GIO module 1 = Power-down mode"]
pub type GiopwdnR = crate::BitReader;
#[doc = "Field `GIOPWDN` writer - 0:0\\]
Writing to the GIOPWDN bit is only allowed in privilege mode. Reading of the GIOPWDN bit is allowed in all modes. Privilege mode (write): 0 = Normal operation; clocks enabled to GIO module 1 = Power-down mode User mode (write): Writes have no effect in user mode. User or privilege mode (read): 0 = Normal operation; clocks enabled to GIO module 1 = Power-down mode"]
pub type GiopwdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:1\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:1\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing to the GIOPWDN bit is only allowed in privilege mode. Reading of the GIOPWDN bit is allowed in all modes. Privilege mode (write): 0 = Normal operation; clocks enabled to GIO module 1 = Power-down mode User mode (write): Writes have no effect in user mode. User or privilege mode (read): 0 = Normal operation; clocks enabled to GIO module 1 = Power-down mode"]
    #[inline(always)]
    pub fn giopwdn(&self) -> GiopwdnR {
        GiopwdnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing to the GIOPWDN bit is only allowed in privilege mode. Reading of the GIOPWDN bit is allowed in all modes. Privilege mode (write): 0 = Normal operation; clocks enabled to GIO module 1 = Power-down mode User mode (write): Writes have no effect in user mode. User or privilege mode (read): 0 = Normal operation; clocks enabled to GIO module 1 = Power-down mode"]
    #[inline(always)]
    #[must_use]
    pub fn giopwdn(&mut self) -> GiopwdnW<GiopwdnSpec> {
        GiopwdnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<GiopwdnSpec> {
        NuW::new(self, 1)
    }
}
#[doc = "GIO power down mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`giopwdn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopwdn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopwdnSpec;
impl crate::RegisterSpec for GiopwdnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopwdn::R`](R) reader structure"]
impl crate::Readable for GiopwdnSpec {}
#[doc = "`write(|w| ..)` method takes [`giopwdn::W`](W) writer structure"]
impl crate::Writable for GiopwdnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPWDN to value 0"]
impl crate::Resettable for GiopwdnSpec {
    const RESET_VALUE: u32 = 0;
}
