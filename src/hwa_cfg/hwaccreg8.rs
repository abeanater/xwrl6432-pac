#[doc = "Register `HWACCREG8` reader"]
pub type R = crate::R<Hwaccreg8Spec>;
#[doc = "Register `HWACCREG8` writer"]
pub type W = crate::W<Hwaccreg8Spec>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `FFTSUMDIV` reader - 28:24\\]
Right-shifting for Sum Statistic: This register specifies how many LSBs to drop to convert the sum statistics to 24-bit value going to the Output Formatter"]
pub type FftsumdivR = crate::FieldReader;
#[doc = "Field `FFTSUMDIV` writer - 28:24\\]
Right-shifting for Sum Statistic: This register specifies how many LSBs to drop to convert the sum statistics to 24-bit value going to the Output Formatter"]
pub type FftsumdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Right-shifting for Sum Statistic: This register specifies how many LSBs to drop to convert the sum statistics to 24-bit value going to the Output Formatter"]
    #[inline(always)]
    pub fn fftsumdiv(&self) -> FftsumdivR {
        FftsumdivR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Hwaccreg8Spec> {
        Nu1W::new(self, 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Right-shifting for Sum Statistic: This register specifies how many LSBs to drop to convert the sum statistics to 24-bit value going to the Output Formatter"]
    #[inline(always)]
    #[must_use]
    pub fn fftsumdiv(&mut self) -> FftsumdivW<Hwaccreg8Spec> {
        FftsumdivW::new(self, 24)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Hwaccreg8Spec> {
        Nu2W::new(self, 29)
    }
}
#[doc = "HWACCREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg8Spec;
impl crate::RegisterSpec for Hwaccreg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg8::R`](R) reader structure"]
impl crate::Readable for Hwaccreg8Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg8::W`](W) writer structure"]
impl crate::Writable for Hwaccreg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG8 to value 0"]
impl crate::Resettable for Hwaccreg8Spec {
    const RESET_VALUE: u32 = 0;
}
