#[doc = "Register `DCEST6I` reader"]
pub type R = crate::R<Dcest6iSpec>;
#[doc = "Register `DCEST6I` writer"]
pub type W = crate::W<Dcest6iSpec>;
#[doc = "Field `DCEST6I` reader - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =5 ."]
pub type Dcest6iR = crate::FieldReader<u32>;
#[doc = "Field `DCEST6I` writer - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =5 ."]
pub type Dcest6iW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =5 ."]
    #[inline(always)]
    pub fn dcest6i(&self) -> Dcest6iR {
        Dcest6iR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =5 ."]
    #[inline(always)]
    #[must_use]
    pub fn dcest6i(&mut self) -> Dcest6iW<Dcest6iSpec> {
        Dcest6iW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcest6iSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "DCEST6I\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest6i::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest6i::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcest6iSpec;
impl crate::RegisterSpec for Dcest6iSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest6i::R`](R) reader structure"]
impl crate::Readable for Dcest6iSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest6i::W`](W) writer structure"]
impl crate::Writable for Dcest6iSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST6I to value 0"]
impl crate::Resettable for Dcest6iSpec {
    const RESET_VALUE: u32 = 0;
}
