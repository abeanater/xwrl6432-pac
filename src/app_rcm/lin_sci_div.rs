#[doc = "Register `LIN_SCI_DIV` reader"]
pub type R = crate::R<LinSciDivSpec>;
#[doc = "Register `LIN_SCI_DIV` writer"]
pub type W = crate::W<LinSciDivSpec>;
#[doc = "Field `val` reader - 7:0\\]
ICG Based Divider for LIN"]
pub type ValR = crate::FieldReader;
#[doc = "Field `val` writer - 7:0\\]
ICG Based Divider for LIN"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
ICG Based Divider for LIN"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
ICG Based Divider for LIN"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<LinSciDivSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "LIN_SCI_DIV\n\nYou can [`read`](crate::Reg::read) this register and get [`lin_sci_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lin_sci_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinSciDivSpec;
impl crate::RegisterSpec for LinSciDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lin_sci_div::R`](R) reader structure"]
impl crate::Readable for LinSciDivSpec {}
#[doc = "`write(|w| ..)` method takes [`lin_sci_div::W`](W) writer structure"]
impl crate::Writable for LinSciDivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIN_SCI_DIV to value 0"]
impl crate::Resettable for LinSciDivSpec {
    const RESET_VALUE: u32 = 0;
}
