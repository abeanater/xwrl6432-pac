#[doc = "Register `DFBCNT1` reader"]
pub type R = crate::R<Dfbcnt1Spec>;
#[doc = "Register `DFBCNT1` writer"]
pub type W = crate::W<Dfbcnt1Spec>;
#[doc = "Field `BCOUNT_REMAINING_FOR_1` reader - 15:0\\]
B-Count Remaining for Dst Register Set:#br#Number of arrays to be transferred where each array is ACNT in length.#br#Represents the amount of data remaining to be written.#br#Initial value is copied from PCNT.#br#TC decrements ACNT and BCNT as necessary after each write dataphase is issued. Final value should be 0 when TR is complete."]
pub type BcountRemainingFor1R = crate::FieldReader<u16>;
#[doc = "Field `BCOUNT_REMAINING_FOR_1` writer - 15:0\\]
B-Count Remaining for Dst Register Set:#br#Number of arrays to be transferred where each array is ACNT in length.#br#Represents the amount of data remaining to be written.#br#Initial value is copied from PCNT.#br#TC decrements ACNT and BCNT as necessary after each write dataphase is issued. Final value should be 0 when TR is complete."]
pub type BcountRemainingFor1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
B-Count Remaining for Dst Register Set:#br#Number of arrays to be transferred where each array is ACNT in length.#br#Represents the amount of data remaining to be written.#br#Initial value is copied from PCNT.#br#TC decrements ACNT and BCNT as necessary after each write dataphase is issued. Final value should be 0 when TR is complete."]
    #[inline(always)]
    pub fn bcount_remaining_for_1(&self) -> BcountRemainingFor1R {
        BcountRemainingFor1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
B-Count Remaining for Dst Register Set:#br#Number of arrays to be transferred where each array is ACNT in length.#br#Represents the amount of data remaining to be written.#br#Initial value is copied from PCNT.#br#TC decrements ACNT and BCNT as necessary after each write dataphase is issued. Final value should be 0 when TR is complete."]
    #[inline(always)]
    #[must_use]
    pub fn bcount_remaining_for_1(&mut self) -> BcountRemainingFor1W<Dfbcnt1Spec> {
        BcountRemainingFor1W::new(self, 0)
    }
}
#[doc = "Dst FIFO Set B-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dfbcnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfbcnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dfbcnt1Spec;
impl crate::RegisterSpec for Dfbcnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfbcnt1::R`](R) reader structure"]
impl crate::Readable for Dfbcnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`dfbcnt1::W`](W) writer structure"]
impl crate::Writable for Dfbcnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFBCNT1 to value 0"]
impl crate::Resettable for Dfbcnt1Spec {
    const RESET_VALUE: u32 = 0;
}
