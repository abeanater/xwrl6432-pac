#[doc = "Register `SABCNT` reader"]
pub type R = crate::R<SabcntSpec>;
#[doc = "Register `SABCNT` writer"]
pub type W = crate::W<SabcntSpec>;
#[doc = "Field `BDIMENSION_COUNT` reader - 15:0\\]
B-Dimension count:#br#Number of arrays to be transferred where each array is ACNT in length.#br#Count Remaining for Src Active Register Set. Represents the amount of data remaining to be read. Initial value is copied from PCNT. TC decrements ACNT and BCNT as necessary after each read command is issued. Final value should be 0 when TR is complete."]
pub type BdimensionCountR = crate::FieldReader<u16>;
#[doc = "Field `BDIMENSION_COUNT` writer - 15:0\\]
B-Dimension count:#br#Number of arrays to be transferred where each array is ACNT in length.#br#Count Remaining for Src Active Register Set. Represents the amount of data remaining to be read. Initial value is copied from PCNT. TC decrements ACNT and BCNT as necessary after each read command is issued. Final value should be 0 when TR is complete."]
pub type BdimensionCountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
B-Dimension count:#br#Number of arrays to be transferred where each array is ACNT in length.#br#Count Remaining for Src Active Register Set. Represents the amount of data remaining to be read. Initial value is copied from PCNT. TC decrements ACNT and BCNT as necessary after each read command is issued. Final value should be 0 when TR is complete."]
    #[inline(always)]
    pub fn bdimension_count(&self) -> BdimensionCountR {
        BdimensionCountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
B-Dimension count:#br#Number of arrays to be transferred where each array is ACNT in length.#br#Count Remaining for Src Active Register Set. Represents the amount of data remaining to be read. Initial value is copied from PCNT. TC decrements ACNT and BCNT as necessary after each read command is issued. Final value should be 0 when TR is complete."]
    #[inline(always)]
    #[must_use]
    pub fn bdimension_count(&mut self) -> BdimensionCountW<SabcntSpec> {
        BdimensionCountW::new(self, 0)
    }
}
#[doc = "Src Actv Set B-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`sabcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sabcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SabcntSpec;
impl crate::RegisterSpec for SabcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sabcnt::R`](R) reader structure"]
impl crate::Readable for SabcntSpec {}
#[doc = "`write(|w| ..)` method takes [`sabcnt::W`](W) writer structure"]
impl crate::Writable for SabcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SABCNT to value 0"]
impl crate::Resettable for SabcntSpec {
    const RESET_VALUE: u32 = 0;
}
